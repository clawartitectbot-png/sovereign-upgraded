/// CodeAgent — Writes code, opens PRs, fixes bugs autonomously
use anyhow::Result;
use crate::config::SovereignConfig;
use super::{AgentTask, AgentPriority, TaskAction};

pub struct CodeAgent;

impl CodeAgent {
    pub async fn poll(config: &SovereignConfig) -> Result<Vec<AgentTask>> {
        let mut tasks = Vec::new();

        // TODO: Check GitHub for open issues assigned to you
        // TODO: Check for failing CI runs in watched repos
        // TODO: Look for new dependency updates

        // Example: check for pending git changes in projects
        tasks.push(AgentTask {
            agent: "CodeAgent".to_string(),
            description: "Scan local projects for uncommitted changes".to_string(),
            priority: AgentPriority::Low,
            action: TaskAction::RunBash(
                "find ~/projects -name '.git' -type d 2>/dev/null | \
                 while read d; do \
                   cd \"$(dirname $d)\" && \
                   git status --short | grep -q '' && \
                   echo \"$(dirname $d): has changes\"; \
                 done".to_string()
            ),
        });

        Ok(tasks)
    }
}
