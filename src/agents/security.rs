/// SecurityAgent — Monitors Pi network, patches vulnerabilities
use anyhow::Result;
use crate::config::SovereignConfig;
use crate::agents::{AgentTask, AgentPriority, TaskAction};

pub struct SecurityAgent;

impl SecurityAgent {
    pub async fn poll(config: &SovereignConfig) -> Result<Vec<AgentTask>> {
        let mut tasks = Vec::new();

        // Check for available system updates
        tasks.push(AgentTask {
            agent: "SecurityAgent".to_string(),
            description: "Check for security updates".to_string(),
            priority: AgentPriority::High,
            action: TaskAction::RunBash(
                "apt-get -s upgrade 2>/dev/null | grep -i 'security' | head -5".to_string()
            ),
        });

        // Check for open ports (basic network scan)
        tasks.push(AgentTask {
            agent: "SecurityAgent".to_string(),
            description: "Scan open ports on Pi".to_string(),
            priority: AgentPriority::Low,
            action: TaskAction::RunBash(
                "ss -tlnp 2>/dev/null | awk 'NR>1 {print $4}' | sort -u".to_string()
            ),
        });

        // Check failed SSH attempts
        tasks.push(AgentTask {
            agent: "SecurityAgent".to_string(),
            description: "Check for failed login attempts".to_string(),
            priority: AgentPriority::Normal,
            action: TaskAction::RunBash(
                "grep 'Failed password' /var/log/auth.log 2>/dev/null | \
                 tail -20 | awk '{print $11}' | sort | uniq -c | sort -rn | head -5".to_string()
            ),
        });

        Ok(tasks)
    }
}
