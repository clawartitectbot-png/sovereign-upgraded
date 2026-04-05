pub mod code;
pub mod income;
pub mod security;
pub mod learning;
pub mod finance;
pub mod upgrade;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Priority levels for agent tasks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentPriority {
    Critical, // Execute now, block everything
    High,     // Execute this tick
    Normal,   // Background execution
    Low,      // Defer to next tick
}

impl PartialOrd for AgentPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AgentPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank = |p: &AgentPriority| match p {
            AgentPriority::Critical => 3,
            AgentPriority::High => 2,
            AgentPriority::Normal => 1,
            AgentPriority::Low => 0,
        };
        rank(self).cmp(&rank(other))
    }
}

/// A unit of work produced by an agent
#[derive(Debug, Clone)]
pub struct AgentTask {
    pub agent: String,
    pub description: String,
    pub priority: AgentPriority,
    pub action: TaskAction,
}

#[derive(Debug, Clone)]
pub enum TaskAction {
    RunBash(String),
    WriteFile { path: String, content: String },
    WebSearch(String),
    OllamaQuery(String),
    Notify(String),
    NoOp,
}

impl AgentTask {
    pub async fn execute(&self) -> Result<()> {
        match &self.action {
            TaskAction::RunBash(cmd) => {
                let output = tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .await?;
                tracing::info!(
                    "    Bash [{}]: {}",
                    if output.status.success() { "ok" } else { "err" },
                    String::from_utf8_lossy(&output.stdout).trim()
                );
            }
            TaskAction::WriteFile { path, content } => {
                if let Some(parent) = std::path::Path::new(path).parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }
                tokio::fs::write(path, content).await?;
                tracing::info!("    Wrote: {}", path);
            }
            TaskAction::Notify(msg) => {
                tracing::info!("    📢 NOTIFY: {}", msg);
                // TODO: integrate with ntfy.sh or local notification
            }
            TaskAction::OllamaQuery(prompt) => {
                tracing::info!("    🤖 Ollama query: {}...", &prompt[..prompt.len().min(50)]);
                // TODO: call ollama client
            }
            TaskAction::WebSearch(query) => {
                tracing::info!("    🔍 Web search: {}", query);
                // TODO: integrate DuckDuckGo API
            }
            TaskAction::NoOp => {}
        }
        Ok(())
    }
}
