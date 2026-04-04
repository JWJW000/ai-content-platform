use crate::models::{Content, CreateTaskRequest, Task};
use crate::repository::{content_repo, task_repo};
use crate::logger;
use crate::service::ai_service;
use anyhow::Result;
use uuid::Uuid;

pub async fn get_all_tasks() -> Result<Vec<Task>> {
    task_repo::get_all_tasks().await
}

pub async fn get_task_by_id(id: Uuid) -> Result<Option<Task>> {
    task_repo::get_task_by_id(id).await
}

pub async fn create_task(req: CreateTaskRequest) -> Result<Task> {
    let task = Task {
        id: Uuid::new_v4(),
        name: req.name,
        platform: req.platform,
        prompt: req.prompt,
        schedule: req.schedule,
        status: "stopped".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    task_repo::insert_task(&task).await?;
    let _ = logger::log_info(Some(task.id), "Task created").await;
    Ok(task)
}

pub async fn start_task(id: Uuid) -> Result<()> {
    let task = task_repo::get_task_by_id(id).await?;
    if let Some(task) = task {
        task_repo::update_task_status(id, "running").await?;
        
        // 将任务添加到调度器
        if let Some(scheduler) = crate::state::get_scheduler().lock().await.as_ref() {
            scheduler.add_task(&task.schedule, id).await?;
        }
        
        let _ = logger::log_info(Some(id), &format!("Task {} started", task.name)).await;
    }
    Ok(())
}

pub async fn stop_task(id: Uuid) -> Result<()> {
    let task = task_repo::get_task_by_id(id).await?;
    if let Some(task) = task {
        task_repo::update_task_status(id, "stopped").await?;
        
        // 从调度器移除任务（标记为不运行）
        let _ = logger::log_info(Some(id), &format!("Task {} stopped", task.name)).await;
    }
    Ok(())
}

pub async fn delete_task(id: Uuid) -> Result<()> {
    // 先停止任务
    let task = task_repo::get_task_by_id(id).await?;
    if let Some(_) = task {
        task_repo::update_task_status(id, "stopped").await?;
    }
    
    task_repo::delete_task(id).await?;
    let _ = logger::log_info(Some(id), "Task deleted").await;
    Ok(())
}

pub async fn generate_content_for_task(task: &Task) -> Result<Content> {
    let ai_content = ai_service::generate_content(&task.prompt, &task.platform).await?;
    
    let content = Content {
        id: Uuid::new_v4(),
        task_id: task.id,
        title: ai_content.title,
        body: ai_content.body,
        status: "pending_review".to_string(),
        score: Some(ai_content.score),
        review_note: None,
        created_at: chrono::Utc::now(),
        reviewed_at: None,
    };
    
    content_repo::insert_content(&content).await?;
    let _ = logger::log_info(Some(task.id), &format!("Content generated: {}", content.title)).await;
    
    Ok(content)
}
