use anyhow::Result;
use axum::{Router, routing::get};

pub async fn serve(port: u16) -> Result<()> {
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/status", get(api_status))
        .route("/api/memory", get(api_memory))
        .route("/api/agents", get(api_agents));

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("🌐 Dashboard running at http://localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn serve_index() -> axum::response::Html<String> {
    let html = tokio::fs::read_to_string("web/index.html")
        .await
        .unwrap_or_else(|_| "<h1>SOVEREIGN running</h1><p>web/index.html not found</p>".to_string());
    axum::response::Html(html)
}

async fn api_status() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "running",
        "version": "0.1.0",
    }))
}

async fn api_memory() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({"files": [], "total_size_kb": 0}))
}

async fn api_agents() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!([
        {"name": "CodeAgent",     "status": "active"},
        {"name": "IncomeAgent",   "status": "active"},
        {"name": "SecurityAgent", "status": "active"},
        {"name": "LearningAgent", "status": "active"},
        {"name": "FinanceAgent",  "status": "active"},
        {"name": "UpgradeAgent",  "status": "active"},
    ]))
}
