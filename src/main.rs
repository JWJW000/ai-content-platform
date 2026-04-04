mod router;
mod handler;
mod service;
mod repository;
mod scheduler;
mod publisher;
mod models;
mod db;
mod logger;
mod state;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .init();

    // 加载环境变量
    dotenvy::dotenv().ok();

    // 初始化数据库连接池
    db::postgres::init_pool().await?;

    // 运行数据库迁移
    db::migrations::run_migrations().await?;

    // 初始化调度器
    let task_scheduler: scheduler::TaskScheduler = scheduler::TaskScheduler::new().await?;
    task_scheduler.start().await?;
    task_scheduler.load_running_tasks().await?;
    
    // 保存调度器到全局状态
    {
        let mut guard = state::get_scheduler().lock().await;
        *guard = Some(task_scheduler);
    }

    tracing::info!("Task scheduler initialized and started");

    // 构建路由
    let app = router::get_routes()
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}
