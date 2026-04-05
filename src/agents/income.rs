/// IncomeAgent — Finds freelance work, tracks earnings
use anyhow::Result;
use crate::config::SovereignConfig;
use crate::agents::{AgentTask, AgentPriority, TaskAction};

pub struct IncomeAgent;

impl IncomeAgent {
    pub async fn poll(config: &SovereignConfig) -> Result<Vec<AgentTask>> {
        let mut tasks = Vec::new();

        // TODO: Query Upwork RSS feed for new jobs matching your skills
        // TODO: Check GitHub bounties board
        // TODO: Monitor crypto portfolio value

        tasks.push(AgentTask {
            agent: "IncomeAgent".to_string(),
            description: "Check freelance opportunity feeds".to_string(),
            priority: AgentPriority::Normal,
            action: TaskAction::WebSearch(
                "site:upwork.com rust developer remote new 24h".to_string()
            ),
        });

        Ok(tasks)
    }
}
