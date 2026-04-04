use crate::db::postgres::get_pool;
use crate::models::Account;
use anyhow::Result;
use uuid::Uuid;

pub async fn get_all_accounts() -> Result<Vec<Account>> {
    let pool = get_pool();
    let accounts = sqlx::query_as::<_, Account>(
        "SELECT id, platform, username, auth, cookie_path, status, created_at FROM accounts ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(accounts)
}

pub async fn get_account_by_id(id: Uuid) -> Result<Option<Account>> {
    let pool = get_pool();
    let account = sqlx::query_as::<_, Account>(
        "SELECT id, platform, username, auth, cookie_path, status, created_at FROM accounts WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(account)
}

pub async fn insert_account(account: &Account) -> Result<()> {
    let pool = get_pool();
    sqlx::query(
        "INSERT INTO accounts (id, platform, username, auth, cookie_path, status, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(account.id)
    .bind(&account.platform)
    .bind(&account.username)
    .bind(&account.auth)
    .bind(&account.cookie_path)
    .bind(&account.status)
    .bind(account.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_account_cookie_path(id: Uuid, cookie_path: &str) -> Result<()> {
    let pool = get_pool();
    sqlx::query(
        "UPDATE accounts SET cookie_path = $1, status = $2 WHERE id = $3"
    )
    .bind(cookie_path)
    .bind("active")
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_account(id: Uuid) -> Result<()> {
    let pool = get_pool();
    sqlx::query("DELETE FROM accounts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
