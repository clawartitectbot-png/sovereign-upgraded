use anyhow::Result;
use chrono::{Local, NaiveTime};
use std::path::{Path, PathBuf};
use tokio::time::{interval, Duration};
use tracing::{info, warn};
use crate::brain::BrainFactory;
use crate::config::SovereignConfig;

pub struct DreamEngine {
    config: SovereignConfig,
    memory_path: PathBuf,
}

impl DreamEngine {
    pub fn new(config: SovereignConfig) -> Self {
        let path = PathBuf::from(
            shellexpand::tilde(&config.memory_path).to_string()
        );
        std::fs::create_dir_all(&path).ok();
        std::fs::create_dir_all(path.join("you")).ok();
        std::fs::create_dir_all(path.join("world")).ok();

        Self {
            config,
            memory_path: path,
        }
    }

    pub async fn run(&self) {
        info!("💤 Dream Engine initialized — will consolidate at {}:00 AM", self.config.dream_hour);

        let mut check_interval = interval(Duration::from_secs(30 * 60));

        loop {
            check_interval.tick().await;
            let now = Local::now();
            let dream_time = NaiveTime::from_hms_opt(self.config.dream_hour, 0, 0).unwrap();
            let current_time = now.time();

            if current_time >= dream_time
                && current_time < dream_time + chrono::Duration::hours(1)
            {
                info!("🌙 Dream Engine activating...");
                if let Err(e) = self.dream().await {
                    warn!("Dream cycle error: {}", e);
                }
                tokio::time::sleep(Duration::from_secs(23 * 60 * 60)).await;
            }
        }
    }

    async fn dream(&self) -> Result<()> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        info!("  Scanning logs for {}...", today);

        let log_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".sovereign")
            .join("logs")
            .join(format!("{}.md", today));

        let log_content = match tokio::fs::read_to_string(&log_path).await {
            Ok(c) => c,
            Err(_) => {
                info!("  No logs for today — skipping dream");
                return Ok(());
            }
        };

        if log_content.trim().is_empty() {
            return Ok(());
        }

        info!("  Consolidating into memory...");

        let prompt = format!(
            "Analyze today's activity log and extract key learnings. Focus on patterns and insights.\n\n\
             TODAY'S LOG:\n{}\n\n\
             EXTRACT LEARNINGS:",
            &log_content[..log_content.len().min(8000)]
        );

        let brain = BrainFactory::get_brain(&self.config);
        let summary = match brain.generate(&prompt, Some("You are SOVEREIGN's memory consolidation system.")).await {
            Ok(s) => s,
            Err(e) => format!("Error consolidating memory: {}", e),
        };

        let consolidation = format!(
            "\n## Dream Cycle — {}\n\n{}\n",
            today, summary
        );

        let memory_file = self.memory_path.join("you").join("daily-consolidations.md");
        let existing = tokio::fs::read_to_string(&memory_file).await.unwrap_or_default();
        tokio::fs::write(&memory_file, format!("{}{}", existing, consolidation)).await?;

        info!("  ✓ Dream complete");
        Ok(())
    }
}
