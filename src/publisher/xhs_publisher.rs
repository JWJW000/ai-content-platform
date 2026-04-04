use crate::models::{Account, Content};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// 小红书发布选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XhsPublishOptions {
    /// 标题
    pub title: String,
    /// 正文内容
    pub content: String,
    /// 话题标签（不含 # 号）
    pub topics: Vec<String>,
    /// 是否为图文笔记（否则为视频）
    pub is_note: bool,
    /// 图片路径列表（仅图文笔记）
    pub image_paths: Option<Vec<String>>,
    /// 发布时间（可选）
    pub publish_at: Option<String>,
}

impl XhsPublishOptions {
    pub fn from_content(content: &Content, topics: Vec<String>) -> Self {
        Self {
            title: content.title.clone(),
            content: content.body.clone(),
            topics,
            is_note: true,
            image_paths: None,
            publish_at: None,
        }
    }
}

/// 小红书发布结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XhsPublishResult {
    /// 是否成功
    pub success: bool,
    /// 状态
    pub status: String,
    /// 消息
    pub message: String,
    /// 笔记 ID（成功后返回）
    pub note_id: Option<String>,
}

impl XhsPublishResult {
    pub fn success(note_id: &str) -> Self {
        Self {
            success: true,
            status: "published".to_string(),
            message: "发布成功".to_string(),
            note_id: Some(note_id.to_string()),
        }
    }

    pub fn failure(msg: &str) -> Self {
        Self {
            success: false,
            status: "failed".to_string(),
            message: msg.to_string(),
            note_id: None,
        }
    }
}

/// 小红书发布器
/// 
/// 集成 rust_drission 进行小红书自动发布
pub struct XiaohongshuPublisher {
    /// rust_drission 路径（当可用时）
    rust_drission_path: Option<String>,
}

impl XiaohongshuPublisher {
    /// 创建新的发布器
    pub fn new() -> Self {
        Self {
            rust_drission_path: None,
        }
    }

    /// 设置 rust_drission 路径
    pub fn with_rust_drission(mut self, path: impl Into<String>) -> Self {
        self.rust_drission_path = Some(path.into());
        self
    }

    /// 发布内容到小红书
    pub async fn publish(&self, content: &Content, account: &Account, options: XhsPublishOptions) -> Result<XhsPublishResult> {
        tracing::info!(
            "Publishing to Xiaohongshu - Title: {}, Account: {}",
            options.title,
            account.username
        );

        // 检查是否有 rust_drission 配置
        if let Some(path) = &self.rust_drission_path {
            self.publish_with_rust_drission(content, account, options, path).await
        } else {
            // 使用模拟模式
            self.publish_mock(content, account, options).await
        }
    }

    /// 使用 rust_drission 真实发布
    async fn publish_with_rust_drission(
        &self, 
        _content: &Content, 
        _account: &Account, 
        _options: XhsPublishOptions,
        _path: &str
    ) -> Result<XhsPublishResult> {
        // TODO: 集成 rust_drission
        // 
        // 示例代码（待实现）：
        // use rust_drission::uploader::{xiaohongshu_upload, ContentType, UploadOptions};
        // 
        // let upload_options = UploadOptions {
        //     title: options.title,
        //     description: Some(options.content),
        //     tags: options.topics.iter().map(|t| format!("#{}", t)).collect(),
        //     content_type: if options.is_note { ContentType::Note } else { ContentType::Video },
        //     image_paths: options.image_paths,
        //     ..Default::default()
        // };
        // 
        // let result = xiaohongshu_upload(&account.auth, upload_options)?;
        
        Err(anyhow::anyhow!("rust_drission integration not yet implemented"))
    }

    /// 模拟发布（用于测试）
    async fn publish_mock(&self, content: &Content, account: &Account, options: XhsPublishOptions) -> Result<XhsPublishResult> {
        // 验证账号
        if account.auth.is_empty() {
            return Ok(XhsPublishResult::failure("Account auth is empty"));
        }

        // 模拟网络延迟
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // 模拟发布
        tracing::info!(
            "Mock publish - Title: {}, Topics: {:?}, Images: {:?}",
            options.title,
            options.topics,
            options.image_paths
        );

        // 生成模拟的笔记 ID
        let note_id = format!("xhs_{}", uuid::Uuid::new_v4().to_string().replace("-", "").chars().take(12).collect::<String>());

        tracing::info!(
            "Content published successfully - Note ID: {}, Account: {}",
            note_id,
            account.username
        );

        Ok(XhsPublishResult::success(&note_id))
    }

    /// 验证账号是否有效
    pub async fn validate_account(&self, account: &Account) -> Result<bool> {
        if account.auth.is_empty() {
            return Ok(false);
        }

        // TODO: 可以调用 rust_drission 的登录验证功能
        // 目前简单检查 auth 是否存在且长度合理
        Ok(account.auth.len() >= 10)
    }
}

impl Default for XiaohongshuPublisher {
    fn default() -> Self {
        Self::new()
    }
}

/// 发布内容到指定平台
pub async fn publish_content(content: &Content, account: &Account, options: XhsPublishOptions) -> Result<XhsPublishResult> {
    let publisher = XiaohongshuPublisher::new();
    publisher.publish(content, account, options).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_publish() {
        let publisher = XiaohongshuPublisher::new();
        
        let content = Content {
            id: uuid::Uuid::new_v4(),
            task_id: uuid::Uuid::new_v4(),
            title: "测试标题".to_string(),
            body: "测试内容正文".to_string(),
            status: "generated".to_string(),
            score: Some(8.5),
            created_at: chrono::Utc::now(),
        };

        let account = Account {
            id: uuid::Uuid::new_v4(),
            platform: "xiaohongshu".to_string(),
            username: "test_user".to_string(),
            auth: "mock_cookie_string_12345".to_string(),
            status: "active".to_string(),
            created_at: chrono::Utc::now(),
        };

        let options = XhsPublishOptions::from_content(&content, vec!["测试".to_string(), "话题".to_string()]);

        let result = publisher.publish(&content, &account, options).await.unwrap();
        
        assert!(result.success);
        assert!(result.note_id.is_some());
    }
}
