/// MemoryDir — File-based persistent memory system
/// Inspired by the memdir system from leaked Claude Code spec §11_special_systems.md
///
/// Memory structure:
///   ~/.sovereign/memory/
///     you/
///       goals.md             ← your objectives, updated daily
///       coding-style.md      ← how you write code
///       income-history.md    ← what has earned money
///       knowledge-gaps.md    ← what you need to learn
///       preferences.md       ← tools, workflows, habits
///       daily-consolidations.md  ← dream engine output
///     world/
///       market-opportunities.md
///       threat-landscape.md
///       model-benchmarks.md

use anyhow::Result;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryFile {
    pub name: String,
    pub path: PathBuf,
    pub content: String,
    pub age_days: i64,
    pub is_stale: bool,
}

pub struct MemoryDir {
    root: PathBuf,
}

impl MemoryDir {
    pub fn new(root: &str) -> Self {
        let path = PathBuf::from(shellexpand::tilde(root).to_string());
        std::fs::create_dir_all(path.join("you")).ok();
        std::fs::create_dir_all(path.join("world")).ok();

        // Seed default memory files if they don't exist
        let defaults = [
            ("you/goals.md", "# Your Goals\n\n<!-- Add your objectives here. SOVEREIGN updates this file. -->\n\n## Current Goals\n- \n\n## Completed\n- \n"),
            ("you/coding-style.md", "# Your Coding Style\n\n<!-- SOVEREIGN observes and fills this in over time -->\n\n## Languages\n- \n\n## Preferred Patterns\n- \n"),
            ("you/income-history.md", "# Income History\n\n<!-- IncomeAgent tracks earnings here -->\n\n| Date | Source | Amount | Notes |\n|------|--------|--------|-------|\n"),
            ("you/knowledge-gaps.md", "# Knowledge Gaps\n\n<!-- LearningAgent identifies what you need to learn -->\n\n## Priority\n- \n\n## Backlog\n- \n"),
            ("you/preferences.md", "# Your Preferences\n\n<!-- SOVEREIGN learns your preferences over time -->\n\n## Tools\n- Editor: \n- Shell: \n\n## Work Style\n- \n"),
            ("world/market-opportunities.md", "# Market Opportunities\n\n<!-- IncomeAgent scans for opportunities here -->\n\n## Active\n- \n"),
            ("world/threat-landscape.md", "# Threat Landscape\n\n<!-- SecurityAgent logs threats here -->\n\n## Active Threats\n- \n\n## Resolved\n- \n"),
            ("world/model-benchmarks.md", "# Model Benchmarks\n\n<!-- UpgradeAgent logs benchmark results here -->\n\n| Date | Model | Tok/s | Score |\n|------|-------|-------|-------|\n"),
        ];

        for (relative, content) in &defaults {
            let file_path = path.join(relative);
            if !file_path.exists() {
                std::fs::write(&file_path, content).ok();
            }
        }

        Self { root: path }
    }

    /// Read all memory files into a context string (for LLM injection)
    pub async fn load_all(&self) -> Result<String> {
        let mut context = String::from("# SOVEREIGN Memory\n\n");
        self.load_dir(&self.root, &mut context).await?;
        Ok(context)
    }

    async fn load_dir(&self, dir: &Path, output: &mut String) -> Result<()> {
        let mut entries = tokio::fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "md") {
                let content = tokio::fs::read_to_string(&path).await.unwrap_or_default();
                let name = path.file_stem().unwrap_or_default().to_string_lossy();
                let age = self.file_age_days(&path).await;
                let stale_warning = if age > 7 {
                    format!(" *(note: {} days old, may be stale)*", age)
                } else {
                    String::new()
                };
                output.push_str(&format!("\n## {}{}\n{}\n", name, stale_warning, content));
            } else if path.is_dir() {
                Box::pin(self.load_dir(&path, output)).await?;
            }
        }
        Ok(())
    }

    /// Append to a specific memory file
    pub async fn append(&self, relative_path: &str, content: &str) -> Result<()> {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let existing = tokio::fs::read_to_string(&path).await.unwrap_or_default();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M").to_string();
        tokio::fs::write(
            &path,
            format!("{}\n<!-- {} -->\n{}\n", existing, timestamp, content),
        )
        .await?;
        Ok(())
    }

    async fn file_age_days(&self, path: &Path) -> i64 {
        if let Ok(metadata) = tokio::fs::metadata(path).await {
            if let Ok(modified) = metadata.modified() {
                let duration = std::time::SystemTime::now()
                    .duration_since(modified)
                    .unwrap_or_default();
                return (duration.as_secs() / 86400) as i64;
            }
        }
        0
    }
}
