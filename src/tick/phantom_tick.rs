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
        let log_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".sovereign")
            .join("logs");
        std::fs::create_dir_all(&log_path).ok();
        Self { config, log_path }
    }

    pub async fn run(&self) -> Result<()> {
        let tick_duration = Duration::from_secs(self.config.tick_interval_minutes * 60);
        let mut ticker = interval(tick_duration);
        loop {
            ticker.tick().await;
            if let Err(e) = self.fire().await {
                warn!("Tick error: {}", e);
            }
        }
    }

    async fn fire(&self) -> Result<()> {
        let now = Local::now();
        info!("🔔 PHANTOM TICK [{}] firing...", now.format("%H:%M:%S"));
        self.append_log(&format!("\n## Tick {}\n", now.format("%Y-%m-%d %H:%M:%S"))).await;

        let mut all_tasks: Vec<AgentTask> = Vec::new();

        if self.config.agents.code_agent {
            match CodeAgent::poll(&self.config).await {
                Ok(t) => all_tasks.extend(t),
                Err(e) => warn!("CodeAgent: {}", e),
            }
        }
        if self.config.agents.income_agent {
            match IncomeAgent::poll(&self.config).await {
                Ok(t) => all_tasks.extend(t),
                Err(e) => warn!("IncomeAgent: {}", e),
            }
        }
        if self.config.agents.security_agent {
            match SecurityAgent::poll(&self.config).await {
                Ok(t) => all_tasks.extend(t),
                Err(e) => warn!("SecurityAgent: {}", e),
            }
        }
        if self.config.agents.learning_agent {
            match LearningAgent::poll(&self.config).await {
                Ok(t) => all_tasks.extend(t),
                Err(e) => warn!("LearningAgent: {}", e),
            }
        }
        if self.config.agents.finance_agent {
            match FinanceAgent::poll(&self.config).await {
                Ok(t) => all_tasks.extend(t),
                Err(e) => warn!("FinanceAgent: {}", e),
            }
        }
        if self.config.agents.upgrade_agent {
            match UpgradeAgent::poll(&self.config).await {
                Ok(t) => all_tasks.extend(t),
                Err(e) => warn!("UpgradeAgent: {}", e),
            }
        }

        all_tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
        info!("  Found {} pending tasks", all_tasks.len());
        self.append_log(&format!("- Tasks: {}\n", all_tasks.len())).await;

        for task in &all_tasks {
            match task.priority {
                AgentPriority::Critical | AgentPriority::High => {
                    info!("  ⚡ Executing: {}", task.description);
                    if let Err(e) = task.execute().await {
                        warn!("Task error: {}", e);
                    }
                }
                AgentPriority::Normal => {
                    let task = task.clone();
                    tokio::spawn(async move {
                        if let Err(e) = task.execute().await {
                            warn!("Background task error: {}", e);
                        }
                    });
                }
                AgentPriority::Low => {}
            }
        }

        info!("  ✓ Tick complete");
        Ok(())
    }

    async fn append_log(&self, content: &str) {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let log_file = self.log_path.join(format!("{}.md", today));
        let existing = tokio::fs::read_to_string(&log_file).await.unwrap_or_default();
        tokio::fs::write(&log_file, format!("{}{}", existing, content)).await.ok();
    }
}
