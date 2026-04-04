use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub static DB_POOL: std::sync::OnceLock<PgPool> = std::sync::OnceLock::new();

pub async fn init_pool() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    
    DB_POOL.set(pool).expect("Failed to set DB pool");
    
    tracing::info!("Database connection pool initialized");
    Ok(())
}

pub fn get_pool() -> &'static PgPool {
    DB_POOL.get().expect("DB pool not initialized")
}
