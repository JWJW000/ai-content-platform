use crate::models::{Account, CreateAccountRequest, LoginResponse};
use crate::repository::account_repo;
use anyhow::Result;
use std::path::PathBuf;
use uuid::Uuid;

/// 获取所有账号
pub async fn get_all_accounts() -> Result<Vec<Account>> {
    account_repo::get_all_accounts().await
}

/// 获取单个账号
pub async fn get_account_by_id(id: Uuid) -> Result<Option<Account>> {
    account_repo::get_account_by_id(id).await
}

/// 创建账号
pub async fn create_account(req: CreateAccountRequest) -> Result<Account> {
    let account = Account {
        id: Uuid::new_v4(),
        platform: req.platform,
        username: req.username,
        auth: req.auth.unwrap_or_default(),
        cookie_path: None,
        status: "pending_login".to_string(), // 新账号需要登录
        created_at: chrono::Utc::now(),
    };
    account_repo::insert_account(&account).await?;
    Ok(account)
}

/// 删除账号
pub async fn delete_account(id: Uuid) -> Result<()> {
    account_repo::delete_account(id).await
}

/// 登录账号（触发浏览器扫码登录）
pub async fn login_account(id: Uuid) -> Result<LoginResponse> {
    let account = account_repo::get_account_by_id(id).await?
        .ok_or_else(|| anyhow::anyhow!("Account not found"))?;
    
    // 构建 cookie 文件路径
    let cookie_dir = get_cookie_dir()?;
    std::fs::create_dir_all(&cookie_dir)?;
    let cookie_path = cookie_dir.join(format!("{}_{}.json", account.platform, account.id));
    let cookie_path_str = cookie_path.to_string_lossy().to_string();
    
    match account.platform.as_str() {
        "xiaohongshu" => login_xiaohongshu(&cookie_path_str).await,
        "douyin" => login_douyin(&cookie_path_str).await,
        _ => Err(anyhow::anyhow!("Unsupported platform: {}", account.platform)),
    }?;
    
    // 更新账号的 cookie 路径
    account_repo::update_account_cookie_path(id, &cookie_path_str).await?;
    
    Ok(LoginResponse {
        success: true,
        message: "Login completed successfully".to_string(),
        qrcode_base64: None,
    })
}

/// 验证账号是否有效
pub async fn verify_account(id: Uuid) -> Result<bool> {
    let account = account_repo::get_account_by_id(id).await?
        .ok_or_else(|| anyhow::anyhow!("Account not found"))?;
    
    // 如果没有 cookie_path，先检查 auth
    if account.cookie_path.is_none() {
        if account.auth.is_empty() {
            return Ok(false);
        }
        // 如果 auth 存在但没有 cookie_path，认为有效
        return Ok(true);
    }
    
    let cookie_path = account.cookie_path.as_ref().unwrap();
    if !std::path::Path::new(cookie_path).exists() {
        return Ok(false);
    }
    
    match account.platform.as_str() {
        "xiaohongshu" => verify_xiaohongshu(cookie_path).await,
        "douyin" => verify_douyin(cookie_path).await,
        _ => Err(anyhow::anyhow!("Unsupported platform: {}", account.platform)),
    }
}

/// 获取 Cookie 存储目录
fn get_cookie_dir() -> Result<PathBuf> {
    let base_dir = std::env::var("COOKIE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("ai-content-platform")
                .join("cookies")
        });
    Ok(base_dir)
}

/// 小红书登录
async fn login_xiaohongshu(cookie_path: &str) -> Result<()> {
    use rust_drission::login::xiaohongshu_login;
    
    tracing::info!("Starting Xiaohongshu login, saving cookies to: {}", cookie_path);
    
    // 调用 rust_drission 的登录功能
    let result = xiaohongshu_login(cookie_path)
        .map_err(|e| anyhow::anyhow!("Xiaohongshu login failed: {:?}", e))?;
    
    if result.success {
        tracing::info!("Xiaohongshu login successful: {}", result.message);
        Ok(())
    } else {
        Err(anyhow::anyhow!("Xiaohongshu login failed: {}", result.message))
    }
}

/// 抖音登录
async fn login_douyin(cookie_path: &str) -> Result<()> {
    use rust_drission::login::douyin_login;
    
    tracing::info!("Starting Douyin login, saving cookies to: {}", cookie_path);
    
    let result = douyin_login(cookie_path)
        .map_err(|e| anyhow::anyhow!("Douyin login failed: {:?}", e))?;
    
    if result.success {
        tracing::info!("Douyin login successful: {}", result.message);
        Ok(())
    } else {
        Err(anyhow::anyhow!("Douyin login failed: {}", result.message))
    }
}

/// 验证小红书账号
async fn verify_xiaohongshu(cookie_path: &str) -> Result<bool> {
    use rust_drission::login::xiaohongshu_verify;
    
    rust_drission::login::xiaohongshu_verify(cookie_path)
        .map_err(|e| anyhow::anyhow!("Xiaohongshu verify failed: {:?}", e))
}

/// 验证抖音账号
async fn verify_douyin(cookie_path: &str) -> Result<bool> {
    use rust_drission::login::douyin_verify;
    
    rust_drission::login::douyin_verify(cookie_path)
        .map_err(|e| anyhow::anyhow!("Douyin verify failed: {:?}", e))
}
