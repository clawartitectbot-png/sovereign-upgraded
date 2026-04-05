/// Web Dashboard — serves the local SOVEREIGN dashboard
/// Accessible at http://localhost:8080

use anyhow::Result;
use axum::{Router, routing::get};
use tower_http::fs::ServeDir;

pub async fn serve(port: u16) -> Result<()> {
    let app = Router::new()
        .route("/api/status", get(api_status))
        .route("/api/memory", get(api_memory))
        .route("/api/agents", get(api_agents))
        .fallback_service(ServeDir::new("web"));

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("🌐 Dashboard running at http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn api_status() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "running",
        "version": "0.1.0",
        "uptime_seconds": 0,
        "next_tick_in": "calculating...",
        "dream_at": "02:00",
    }))
}

async fn api_memory() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "files": [],
        "total_size_kb": 0,
        "last_dream": null,
    }))
}

async fn api_agents() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!([
        {"name": "CodeAgent",     "status": "active", "last_run": null, "tasks_today": 0},
        {"name": "IncomeAgent",   "status": "active", "last_run": null, "tasks_today": 0},
        {"name": "SecurityAgent", "status": "active", "last_run": null, "tasks_today": 0},
        {"name": "LearningAgent", "status": "active", "last_run": null, "tasks_today": 0},
        {"name": "FinanceAgent",  "status": "active", "last_run": null, "tasks_today": 0},
        {"name": "UpgradeAgent",  "status": "active", "last_run": null, "tasks_today": 0},
    ]))
}
