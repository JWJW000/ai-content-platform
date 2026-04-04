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
pub struct XiaohongshuPublisher;

impl XiaohongshuPublisher {
    /// 创建新的发布器
    pub fn new() -> Self {
        Self
    }

    /// 发布内容到小红书
    pub async fn publish(&self, content: &Content, account: &Account, options: XhsPublishOptions) -> Result<XhsPublishResult> {
        tracing::info!(
            "Publishing to Xiaohongshu - Title: {}, Account: {}",
            options.title,
            account.username
        );

        // 获取 cookie 文件路径
        let cookie_path = self.get_cookie_path(account)?;
        
        if cookie_path.is_empty() || !std::path::Path::new(&cookie_path).exists() {
            return Ok(XhsPublishResult::failure("Account not logged in or cookie file not found"));
        }

        self.publish_with_rust_drission(content, account, options, &cookie_path).await
    }

    /// 获取账号的 cookie 文件路径
    fn get_cookie_path(&self, account: &Account) -> Result<String> {
        // 优先使用 cookie_path
        if let Some(ref path) = account.cookie_path {
            if !path.is_empty() {
                return Ok(path.clone());
            }
        }
        
        // 如果没有 cookie_path，使用 auth 字段（兼容旧数据）
        if !account.auth.is_empty() && account.auth.starts_with('/') {
            return Ok(account.auth.clone());
        }
        
        Ok(String::new())
    }

    /// 使用 rust_drission 真实发布
    async fn publish_with_rust_drission(
        &self, 
        content: &Content, 
        account: &Account, 
        options: XhsPublishOptions,
        cookie_path: &str
    ) -> Result<XhsPublishResult> {
        use rust_drission::uploader::{XiaoHongShuUploader, ContentType, UploadResult as DrissionUploadResult};
        
        tracing::info!(
            "Using rust_drission to publish - Title: {}, Cookie: {}",
            options.title,
            cookie_path
        );

        // 构建上传器
        let mut uploader = XiaoHongShuUploader::new(cookie_path)
            .title(&options.title)
            .description(&options.content)
            .tags(options.topics.clone())
            .content_type(if options.is_note { ContentType::Note } else { ContentType::Video })
            .headless(false);  // 需要人机交互时显示浏览器

        // 设置图片（仅图文模式）
        if options.is_note {
            if let Some(ref image_paths) = options.image_paths {
                if !image_paths.is_empty() {
                    uploader = uploader.image_paths(image_paths.clone());
                }
            }
        }

        // 执行上传
        let result = uploader.upload()
            .map_err(|e| anyhow::anyhow!("Upload failed: {:?}", e))?;

        if result.success {
            tracing::info!(
                "Content published successfully - Note ID: {}, Account: {}",
                result.video_id.as_deref().unwrap_or("unknown"),
                account.username
            );
            Ok(XhsPublishResult {
                success: true,
                status: result.status,
                message: result.message,
                note_id: result.video_id,
            })
        } else {
            tracing::error!(
                "Failed to publish - Status: {}, Message: {}, Account: {}",
                result.status,
                result.message,
                account.username
            );
            Ok(XhsPublishResult::failure(&result.message))
        }
    }

    /// 验证账号是否有效
    pub async fn validate_account(&self, account: &Account) -> Result<bool> {
        if let Some(ref path) = account.cookie_path {
            if !path.is_empty() && std::path::Path::new(path).exists() {
                use rust_drission::login::xiaohongshu_verify;
                return xiaohongshu_verify(path)
                    .map_err(|e| anyhow::anyhow!("Verify failed: {:?}", e));
            }
        }
        
        if !account.auth.is_empty() && account.auth.starts_with('/') {
            use rust_drission::login::xiaohongshu_verify;
            return xiaohongshu_verify(&account.auth)
                .map_err(|e| anyhow::anyhow!("Verify failed: {:?}", e));
        }
        
        Ok(false)
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
    async fn test_publish_without_cookie() {
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
            auth: String::new(),
            cookie_path: None,
            status: "active".to_string(),
            created_at: chrono::Utc::now(),
        };

        let options = XhsPublishOptions::from_content(&content, vec!["测试".to_string()]);

        let result = publisher.publish(&content, &account, options).await.unwrap();
        
        // 没有 cookie 文件应该返回失败
        assert!(!result.success);
    }
}
