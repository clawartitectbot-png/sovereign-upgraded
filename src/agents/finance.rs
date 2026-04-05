/// FinanceAgent — Tracks income/expenses, tax prep, budget alerts
use anyhow::Result;
use crate::config::SovereignConfig;
use crate::agents::{AgentTask, AgentPriority, TaskAction};

pub struct FinanceAgent;

impl FinanceAgent {
    pub async fn poll(config: &SovereignConfig) -> Result<Vec<AgentTask>> {
        let mut tasks = Vec::new();

        // Daily finance log entry
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let finance_log = format!("~/.sovereign/memory/you/finance-log.md");

        tasks.push(AgentTask {
            agent: "FinanceAgent".to_string(),
            description: "Update daily finance log".to_string(),
            priority: AgentPriority::Low,
            action: TaskAction::WriteFile {
                path: shellexpand::tilde(&finance_log).to_string(),
                content: format!(
                    "<!-- FinanceAgent auto-log — add your income/expenses below -->\n\
                     ## {}\n\
                     - Income: \n\
                     - Expenses: \n\
                     - Notes: \n",
                    today
                ),
            },
        });

        Ok(tasks)
    }
}
