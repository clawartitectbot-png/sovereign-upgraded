/// PHANTOM TICK — Autonomous Decision Loop
///
/// Inspired by KAIROS from the leaked Claude Code source.
/// Fires every N minutes, evaluates the world state, dispatches agents.
///
/// Original KAIROS pattern:
///   - Receives periodic <tick> prompts
///   - Maintains append-only daily log files
///   - Decides autonomously whether to act or wait
///   - Sends push notifications to user when needed
///
/// SOVEREIGN's PHANTOM TICK:
///   - Runs every 15 minutes (configurable)
///   - Queries each active agent for pending actions
///   - Executes high-priority actions immediately
///   - Logs everything to ~/.sovereign/logs/YYYY-MM-DD.md

use anyhow::Result;
use chrono::Local;
use std::path::PathBuf;
use tokio::time::{interval, Duration};
use tracing::{info, warn};

use crate::config::SovereignConfig;
use crate::agents::{
    AgentTask, AgentPriority,
    code::CodeAgent,
    income::IncomeAgent,
    security::SecurityAgent,
    learning::LearningAgent,
    finance::FinanceAgent,
    upgrade::UpgradeAgent,
};

pub struct PhantomTick {
    config: SovereignConfig,
    log_path: PathBuf,
}

impl PhantomTick {
    pub fn new(config: SovereignConfig) -> Self {
        let log_path = PathBuf::from(
            shellexpand::tilde("~/.sovereign/logs").to_string()
        );
        std::fs::create_dir_all(&log_path).ok();
        Self { config, log_path }
    }

    /// Main tick loop — runs forever
    pub async fn run(&self) -> Result<()> {
        let tick_duration = Duration::from_secs(self.config.tick_interval_minutes * 60);
        let mut ticker = interval(tick_duration);

        // First tick fires immediately on startup
        loop {
            ticker.tick().await;
            if let Err(e) = self.fire().await {
                warn!("Tick error: {}", e);
            }
        }
    }

    /// Single tick execution
    async fn fire(&self) -> Result<()> {
        let now = Local::now();
        let tick_id = now.format("%Y%m%d_%H%M%S").to_string();

        info!("🔔 PHANTOM TICK [{}] firing...", tick_id);
        self.log(&format!("\n## Tick {}\n", now.format("%Y-%m-%d %H:%M:%S"))).await;

        let mut all_tasks: Vec<AgentTask> = Vec::new();

        // Poll each enabled agent for pending tasks
        if self.config.agents.code_agent {
            match CodeAgent::poll(&self.config).await {
                Ok(tasks) => all_tasks.extend(tasks),
                Err(e) => warn!("CodeAgent poll error: {}", e),
            }
        }

        if self.config.agents.income_agent {
            match IncomeAgent::poll(&self.config).await {
                Ok(tasks) => all_tasks.extend(tasks),
                Err(e) => warn!("IncomeAgent poll error: {}", e),
            }
        }

        if self.config.agents.security_agent {
            match SecurityAgent::poll(&self.config).await {
                Ok(tasks) => all_tasks.extend(tasks),
                Err(e) => warn!("SecurityAgent poll error: {}", e),
            }
        }

        if self.config.agents.learning_agent {
            match LearningAgent::poll(&self.config).await {
                Ok(tasks) => all_tasks.extend(tasks),
                Err(e) => warn!("LearningAgent poll error: {}", e),
            }
        }

        if self.config.agents.finance_agent {
            match FinanceAgent::poll(&self.config).await {
                Ok(tasks) => all_tasks.extend(tasks),
                Err(e) => warn!("FinanceAgent poll error: {}", e),
            }
        }

        if self.config.agents.upgrade_agent {
            match UpgradeAgent::poll(&self.config).await {
                Ok(tasks) => all_tasks.extend(tasks),
                Err(e) => warn!("UpgradeAgent poll error: {}", e),
            }
        }

        // Sort by priority: Critical > High > Normal > Low
        all_tasks.sort_by(|a, b| b.priority.cmp(&a.priority));

        info!("  Found {} pending tasks", all_tasks.len());
        self.log(&format!("- **Tasks found:** {}\n", all_tasks.len())).await;

        // Execute tasks
        for task in &all_tasks {
            match task.priority {
                AgentPriority::Critical | AgentPriority::High => {
                    info!("  ⚡ [{:?}] Executing: {}", task.priority, task.description);
                    self.log(&format!("- ⚡ `{}`: {}\n", task.agent, task.description)).await;
                    if let Err(e) = task.execute().await {
                        warn!("Task execution error: {}", e);
                    }
                }
                AgentPriority::Normal => {
                    info!("  → [{:?}] Queued: {}", task.priority, task.description);
                    self.log(&format!("- → `{}`: {}\n", task.agent, task.description)).await;
                    // Queue for background execution
                    tokio::spawn({
                        let task = task.clone();
                        async move {
                            if let Err(e) = task.execute().await {
                                warn!("Background task error: {}", e);
                            }
                        }
                    });
                }
                AgentPriority::Low => {
                    // Just log, execute next tick
                    self.log(&format!("- ○ `{}`: {} (deferred)\n", task.agent, task.description)).await;
                }
            }
        }

        info!("  ✓ Tick complete");
        Ok(())
    }

    /// Append to today's daily log
    async fn log(&self, content: &str) {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let log_file = self.log_path.join(format!("{}.md", today));
        if let Err(e) = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .await
            .and_then(|mut f| {
                use tokio::io::AsyncWriteExt;
                Box::pin(async move { f.write_all(content.as_bytes()).await })
            })
            .await
        {
            warn!("Log write error: {}", e);
        }
    }
}
