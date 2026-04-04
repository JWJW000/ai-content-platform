use crate::models::{Account, CreateAccountRequest};
use crate::repository::account_repo;
use anyhow::Result;
use uuid::Uuid;

pub async fn get_all_accounts() -> Result<Vec<Account>> {
    account_repo::get_all_accounts().await
}

pub async fn get_account_by_id(id: Uuid) -> Result<Option<Account>> {
    account_repo::get_account_by_id(id).await
}

pub async fn create_account(req: CreateAccountRequest) -> Result<Account> {
    let account = Account {
        id: Uuid::new_v4(),
        platform: req.platform,
        username: req.username,
        auth: req.auth,
        status: "active".to_string(),
        created_at: chrono::Utc::now(),
    };
    account_repo::insert_account(&account).await?;
    Ok(account)
}

pub async fn delete_account(id: Uuid) -> Result<()> {
    account_repo::delete_account(id).await
}
