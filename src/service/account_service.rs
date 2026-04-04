use crate::models::{Account, CreateAccountRequest, LoginResponse};
use crate::repository::account_repo;
use anyhow::Result;
use std::path::PathBuf;
use uuid::Uuid;

/// 创建账号响应
#[derive(Debug, serde::Serialize)]
pub struct CreateAccountResponse {
    pub account: Account,
    pub login_triggered: bool,
    pub message: String,
}

/// 获取所有账号
pub async fn get_all_accounts() -> Result<Vec<Account>> {
    account_repo::get_all_accounts().await
}

/// 获取单个账号
pub async fn get_account_by_id(id: Uuid) -> Result<Option<Account>> {
    account_repo::get_account_by_id(id).await
}

/// 创建账号（自动触发登录流程）
pub async fn create_account(req: CreateAccountRequest) -> Result<CreateAccountResponse> {
    let account = Account {
        id: Uuid::new_v4(),
        platform: req.platform.clone(),
        username: req.username.clone(),
        auth: String::new(),
        cookie_path: None,
        status: "pending_login".to_string(),
        created_at: chrono::Utc::now(),
    };
    account_repo::insert_account(&account).await?;
    
    // 自动触发登录流程
    let login_result = trigger_login(&account).await;
    
    match login_result {
        Ok(_) => {
            // 更新账号状态为已登录
            account_repo::update_account_status(&account.id, "active").await?;
            
            let logged_in_account = account_repo::get_account_by_id(account.id).await?.unwrap();
            
            Ok(CreateAccountResponse {
                account: logged_in_account,
                login_triggered: true,
                message: "Account created and login completed".to_string(),
            })
        }
        Err(e) => {
            // 登录失败，返回账号但标记需要登录
            Ok(CreateAccountResponse {
                account,
                login_triggered: false,
                message: format!("Account created but login failed: {}. Please trigger login manually.", e),
            })
        }
    }
}

/// 删除账号
pub async fn delete_account(id: Uuid) -> Result<()> {
    account_repo::delete_account(id).await
}

/// 登录账号（触发浏览器扫码登录）
pub async fn login_account(id: Uuid) -> Result<LoginResponse> {
    let account = account_repo::get_account_by_id(id).await?
        .ok_or_else(|| anyhow::anyhow!("Account not found"))?;
    
    trigger_login(&account).await?;
    
    // 更新账号状态
    account_repo::update_account_status(&id, "active").await?;
    
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

/// 触发登录流程
async fn trigger_login(account: &Account) -> Result<()> {
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
    account_repo::update_account_cookie_path(account.id, &cookie_path_str).await?;
    
    Ok(())
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
