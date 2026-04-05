/// BashTool — Execute shell commands safely
/// Risk classification: LOW (read) / MEDIUM (write) / HIGH (system)
/// From claurst spec §BashTool

use anyhow::Result;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

pub struct BashTool;

impl BashTool {
    /// Run a shell command with timeout (default 30s)
    pub async fn run(cmd: &str) -> Result<String> {
        Self::run_with_timeout(cmd, 30).await
    }

    pub async fn run_with_timeout(cmd: &str, seconds: u64) -> Result<String> {
        let output = timeout(
            Duration::from_secs(seconds),
            Command::new("sh").arg("-c").arg(cmd).output(),
        )
        .await??;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() && !stderr.is_empty() {
            return Err(anyhow::anyhow!("Command failed: {}", stderr.trim()));
        }

        Ok(stdout.trim().to_string())
    }

    /// Check if a command exists on the system
    pub async fn command_exists(cmd: &str) -> bool {
        Self::run(&format!("command -v {} 2>/dev/null", cmd))
            .await
            .map(|o| !o.is_empty())
            .unwrap_or(false)
    }
}
