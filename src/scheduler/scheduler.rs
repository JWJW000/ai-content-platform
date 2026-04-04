use crate::db::postgres::get_pool;
use crate::logger;
use crate::models::Execution;
use crate::repository::{content_repo, task_repo, account_repo};
use anyhow::Result;
use cron::Schedule;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

pub struct TaskScheduler {
    scheduler: JobScheduler,
    running_tasks: Arc<Mutex<std::collections::HashMap<Uuid, bool>>>,
}

impl TaskScheduler {
    pub async fn new() -> anyhow::Result<Self> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self {
            scheduler,
            running_tasks: Arc::new(Mutex::new(std::collections::HashMap::new())),
        })
    }

    /// 启动调度器
    pub async fn start(&self) -> anyhow::Result<()> {
        self.scheduler.start().await?;
        tracing::info!("Task scheduler started");
        Ok(())
    }

    /// 加载所有 running 状态的任务
    pub async fn load_running_tasks(&self) -> anyhow::Result<()> {
        let tasks = task_repo::get_running_tasks().await?;
        
        for task in tasks {
            self.add_task(&task.schedule, task.id).await?;
            tracing::info!("Loaded running task: {} ({})", task.name, task.id);
        }
        
        Ok(())
    }

    /// 添加一个定时任务
    pub async fn add_task(&self, cron_expr: &str, task_id: Uuid) -> anyhow::Result<()> {
        // 确保 cron 表达式有 6 个字段（秒 分 时 日 月 周）
        let full_cron = if cron_expr.split_whitespace().count() == 5 {
            format!("0 {}", cron_expr)
        } else {
            cron_expr.to_string()
        };
        
        let _schedule = Schedule::from_str(&full_cron)
            .map_err(|e| anyhow::anyhow!("Invalid cron expression: {}", e))?;
        
        let task_id_clone = task_id;
        let running_tasks = self.running_tasks.clone();
        
        let job = Job::new_async(&full_cron, move |_uuid, _lock| {
            let task_id = task_id_clone;
            let running_tasks = running_tasks.clone();
            Box::pin(async move {
                // 检查任务是否正在运行
                {
                    let mut running = running_tasks.lock().await;
                    if *running.get(&task_id).unwrap_or(&false) {
                        tracing::warn!("Task {} is already running, skipping", task_id);
                        return;
                    }
                    running.insert(task_id, true);
                }
                
                // 执行任务
                if let Err(e) = execute_task(task_id).await {
                    tracing::error!("Task {} execution failed: {}", task_id, e);
                    let _ = logger::log_error(Some(task_id), &format!("Execution failed: {}", e));
                }
                
                // 标记任务结束
                {
                    let mut running = running_tasks.lock().await;
                    running.insert(task_id, false);
                }
            })
        })?;
        
        self.scheduler.add(job).await?;
        tracing::info!("Added scheduled task: {} with schedule {}", task_id, cron_expr);
        
        Ok(())
    }

    /// 移除一个定时任务
    pub async fn remove_task(&self, task_id: Uuid) -> anyhow::Result<()> {
        tracing::info!("Task {} removed from scheduler", task_id);
        Ok(())
    }
}

/// 执行单个任务
pub async fn execute_task(task_id: Uuid) -> Result<()> {
    let pool = get_pool();
    
    // 记录开始
    let execution_id = Uuid::new_v4();
    let execution = Execution {
        id: execution_id,
        task_id,
        status: "running".to_string(),
        started_at: chrono::Utc::now(),
        finished_at: None,
    };
    
    sqlx::query(
        "INSERT INTO executions (id, task_id, status, started_at) VALUES ($1, $2, $3, $4)"
    )
    .bind(execution.id)
    .bind(execution.task_id)
    .bind(&execution.status)
    .bind(execution.started_at)
    .execute(pool)
    .await?;
    
    logger::log_info(Some(task_id), "Task execution started").await?;
    
    // 获取任务信息
    let task = match task_repo::get_task_by_id(task_id).await? {
        Some(t) => t,
        None => {
            logger::log_error(Some(task_id), "Task not found").await?;
            return Ok(());
        }
    };
    
    // 生成内容
    let ai_content = crate::service::ai_service::generate_content(&task.prompt, &task.platform).await?;
    
    let content = crate::models::Content {
        id: Uuid::new_v4(),
        task_id,
        title: ai_content.title.clone(),
        body: ai_content.body.clone(),
        status: "pending_review".to_string(),
        score: Some(ai_content.score),
        review_note: None,
        created_at: chrono::Utc::now(),
        reviewed_at: None,
    };
    
    content_repo::insert_content(&content).await?;
    logger::log_info(Some(task_id), &format!("Content generated and pending review: {}", content.title)).await?;
    logger::log_info(Some(task_id), "Content is pending review, will be published after approval").await?;
    
    // 注意：内容生成后需要审核才能发布，这是审核工作流的一部分
    match &task.platform[..] {
        "xiaohongshu" => {
            // 只有 approved 状态的内容才发布
            if content.status == "approved" {
                let accounts = account_repo::get_all_accounts().await?;
                if let Some(account) = accounts.into_iter().find(|a| a.platform == "xiaohongshu") {
                    let options = crate::publisher::XhsPublishOptions::from_content(&content, vec![]);
                    match crate::publisher::publish_content(&content, &account, options).await {
                        Ok(result) => {
                            if result.success {
                                content_repo::update_content_status(content.id, "published").await?;
                                logger::log_info(Some(task_id), &format!("Content published: {}", result.note_id.unwrap_or_default())).await?;
                            } else {
                                content_repo::update_content_status(content.id, "failed").await?;
                                logger::log_error(Some(task_id), &format!("Publish failed: {}", result.message)).await?;
                            }
                        }
                        Err(e) => {
                            logger::log_error(Some(task_id), &format!("Publish error: {}", e)).await?;
                            content_repo::update_content_status(content.id, "failed").await?;
                        }
                    }
                } else {
                    logger::log_warn(Some(task_id), "No Xiaohongshu account found, skipping publish").await?;
                }
            } else {
                logger::log_info(Some(task_id), &format!("Content {} is not approved yet, skipping publish", content.id)).await?;
            }
        }
        _ => {
            logger::log_warn(Some(task_id), &format!("Platform {} not supported yet", task.platform)).await?;
        }
    }
    
    // 更新执行记录
    sqlx::query("UPDATE executions SET status = 'success', finished_at = NOW() WHERE id = $1")
        .bind(execution_id)
        .execute(pool)
        .await?;
    
    Ok(())
}
