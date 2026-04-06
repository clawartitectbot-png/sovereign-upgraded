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

        // Example: check for pending git changes in projects and auto-commit small ones
        tasks.push(AgentTask {
            agent: "CodeAgent".to_string(),
            description: "Git Checkpoint: Scan and commit small changes in local projects".to_string(),
            priority: AgentPriority::Low,
            action: TaskAction::RunBash(
                "find ~/projects -name '.git' -type d -maxdepth 3 2>/dev/null | \
                 while read d; do \
                   repo_dir=\"$(dirname $d)\" && \
                   cd \"$repo_dir\" && \
                   if [ -n \"$(git status --short)\" ]; then \
                     echo \"Checking $repo_dir...\" && \
                     git add . && \
                     git commit -m \"Sovereign auto-checkpoint: $(date +'%Y-%m-%d %H:%M')\" && \
                     git push origin $(git rev-parse --abbrev-ref HEAD) || true; \
                   fi; \
                 done".to_string()
            ),
        });

        Ok(tasks)
    }
}
