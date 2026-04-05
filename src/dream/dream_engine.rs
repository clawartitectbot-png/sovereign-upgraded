/// DREAM ENGINE — Nightly Memory Consolidation
///
/// Inspired by autoDream from the leaked Claude Code source:
///   SESSION_SCAN_INTERVAL_MS = 10 * 60 * 1000 (10 minutes)
///   Gate: time check (minHours) → session count → consolidation lock
///   Spawns a forked agent using buildConsolidationPrompt()
///
/// SOVEREIGN's Dream Engine:
///   - Runs at 2AM every night (configurable)
///   - Scans today's PHANTOM TICK logs
///   - Asks the local LLM to extract key learnings
///   - Consolidates into persistent memory/*.md files
///   - Builds a growing knowledge graph about YOU specifically
///
/// Memory structure (from leaked memdir spec):
///   ~/.sovereign/memory/
///     you/
///       coding-style.md       ← how you write code
///       income-history.md     ← what's earned money
///       knowledge-gaps.md     ← what you need to learn
///       goals.md              ← your objectives, updated daily
///       preferences.md        ← your tool/workflow preferences
///     world/
///       market-opportunities.md
///       threat-landscape.md
///       model-benchmarks.md

use anyhow::Result;
use chrono::{Local, NaiveTime};
use std::path::{Path, PathBuf};
use tokio::time::{interval, Duration};
use tracing::{info, warn};

pub struct DreamEngine {
    memory_path: PathBuf,
    dream_hour: u32,
}

impl DreamEngine {
    pub fn new(memory_path: String) -> Self {
        let path = PathBuf::from(
            shellexpand::tilde(&memory_path).to_string()
        );
        std::fs::create_dir_all(&path).ok();
        std::fs::create_dir_all(path.join("you")).ok();
        std::fs::create_dir_all(path.join("world")).ok();

        Self {
            memory_path: path,
            dream_hour: 2,
        }
    }

    /// Run the nightly dream loop forever
    pub async fn run(&self) {
        info!("💤 Dream Engine initialized — will consolidate at {}:00 AM", self.dream_hour);

        // Check every 30 minutes if it's dream time
        let mut check_interval = interval(Duration::from_secs(30 * 60));

        loop {
            check_interval.tick().await;
            let now = Local::now();
            let dream_time = NaiveTime::from_hms_opt(self.dream_hour, 0, 0).unwrap();
            let current_time = now.time();

            // Trigger within the 2AM hour
            if current_time >= dream_time
                && current_time < dream_time + chrono::Duration::hours(1)
            {
                info!("🌙 Dream Engine activating...");
                if let Err(e) = self.dream().await {
                    warn!("Dream cycle error: {}", e);
                }
                // Wait until next night
                tokio::time::sleep(Duration::from_secs(23 * 60 * 60)).await;
            }
        }
    }

    /// One dream cycle — consolidate today into memory
    async fn dream(&self) -> Result<()> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        info!("  Scanning logs for {}...", today);

        // Read today's tick logs
        let log_path = PathBuf::from(
            shellexpand::tilde("~/.sovereign/logs").to_string()
        ).join(format!("{}.md", today));

        let log_content = match tokio::fs::read_to_string(&log_path).await {
            Ok(c) => c,
            Err(_) => {
                info!("  No logs for today — skipping dream");
                return Ok(());
            }
        };

        if log_content.trim().is_empty() {
            info!("  Empty log — nothing to dream about");
            return Ok(());
        }

        info!("  Consolidating {} chars of logs into memory...", log_content.len());

        // Consolidation prompt (inspired by buildConsolidationPrompt from spec)
        let prompt = format!(
            "You are SOVEREIGN's memory consolidation system. \
             Analyze today's activity log and extract key learnings, patterns, and insights. \
             Be concise. Focus on: what worked, what failed, new opportunities discovered, \
             security threats found, income earned, knowledge gained. \
             Format as bullet points grouped by category.\n\n\
             TODAY'S LOG:\n{}\n\n\
             EXTRACT LEARNINGS:",
            &log_content[..log_content.len().min(8000)]
        );

        // Write consolidation to memory (in production, this calls Ollama)
        let consolidation = format!(
            "\n## Dream Cycle — {}\n\n\
             *[Auto-consolidated from {} chars of activity logs]*\n\n\
             <!-- Ollama will populate this with actual insights -->\n\
             {}\n",
            today,
            log_content.len(),
            "- Awaiting Ollama connection for real consolidation"
        );

        // Append to memory file
        let memory_file = self.memory_path.join("you").join("daily-consolidations.md");
        let existing = tokio::fs::read_to_string(&memory_file).await.unwrap_or_default();
        tokio::fs::write(&memory_file, format!("{}{}", existing, consolidation)).await?;

        info!("  ✓ Dream complete — memory updated");
        Ok(())
    }

    /// Read all memory files, return as context string
    pub async fn recall(&self, query: &str) -> Result<String> {
        let mut context = String::new();
        self.read_memory_dir(&self.memory_path, &mut context).await?;
        Ok(context)
    }

    async fn read_memory_dir(&self, dir: &Path, output: &mut String) -> Result<()> {
        let mut entries = tokio::fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "md") {
                if let Ok(content) = tokio::fs::read_to_string(&path).await {
                    output.push_str(&format!(
                        "\n--- {} ---\n{}\n",
                        path.file_name().unwrap_or_default().to_string_lossy(),
                        content
                    ));
                }
            } else if path.is_dir() {
                Box::pin(self.read_memory_dir(&path, output)).await?;
            }
        }
        Ok(())
    }
}
