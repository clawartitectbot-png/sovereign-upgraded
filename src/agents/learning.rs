/// LearningAgent — Reads papers/news, fills knowledge gaps
use anyhow::Result;
use crate::config::SovereignConfig;
use crate::agents::{AgentTask, AgentPriority, TaskAction};

pub struct LearningAgent;

impl LearningAgent {
    pub async fn poll(config: &SovereignConfig) -> Result<Vec<AgentTask>> {
        let mut tasks = Vec::new();

        // Check HackerNews top stories
        tasks.push(AgentTask {
            agent: "LearningAgent".to_string(),
            description: "Fetch HackerNews top 5 for knowledge base".to_string(),
            priority: AgentPriority::Low,
            action: TaskAction::RunBash(
                "curl -s 'https://hacker-news.firebaseio.com/v0/topstories.json' 2>/dev/null | \
                 python3 -c 'import json,sys; ids=json.load(sys.stdin)[:5]; print(ids)'".to_string()
            ),
        });

        // Check disk space for knowledge base
        tasks.push(AgentTask {
            agent: "LearningAgent".to_string(),
            description: "Check knowledge base disk usage".to_string(),
            priority: AgentPriority::Low,
            action: TaskAction::RunBash(
                "du -sh ~/.sovereign/memory/ 2>/dev/null || echo '0 (no memory yet)'".to_string()
            ),
        });

        Ok(tasks)
    }
}
