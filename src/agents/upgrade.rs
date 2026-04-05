/// UpgradeAgent — Benchmarks Ollama models, promotes the best one
///
/// Inspired by Project NOMAD's benchmark_service.ts:
///   - AI benchmark model: llama3.2:1b
///   - Benchmark prompt: "Explain recursion in programming in exactly 100 words."
///   - Score weights: ai_tokens_per_second (0.30), cpu (0.25), memory (0.15), etc.
///   - HMAC-signed submission to community leaderboard

use anyhow::Result;
use crate::config::SovereignConfig;
use crate::agents::{AgentTask, AgentPriority, TaskAction};

pub struct UpgradeAgent;

impl UpgradeAgent {
    pub async fn poll(config: &SovereignConfig) -> Result<Vec<AgentTask>> {
        let mut tasks = Vec::new();

        // Check if Ollama is running
        tasks.push(AgentTask {
            agent: "UpgradeAgent".to_string(),
            description: "Check Ollama health and list installed models".to_string(),
            priority: AgentPriority::Normal,
            action: TaskAction::RunBash(
                format!(
                    "curl -s {}/api/tags 2>/dev/null | \
                     python3 -c \"import json,sys; d=json.load(sys.stdin); \
                     [print(m['name']) for m in d.get('models',[])]\" 2>/dev/null || \
                     echo 'Ollama not running'",
                    config.ollama_url
                )
            ),
        });

        // Check disk space before auto-downloading new models
        tasks.push(AgentTask {
            agent: "UpgradeAgent".to_string(),
            description: "Check available disk space for model storage".to_string(),
            priority: AgentPriority::Low,
            action: TaskAction::RunBash(
                "df -h ~/.ollama 2>/dev/null || df -h ~ | tail -1".to_string()
            ),
        });

        // Run a quick benchmark on the primary model
        tasks.push(AgentTask {
            agent: "UpgradeAgent".to_string(),
            description: "Benchmark primary model response time".to_string(),
            priority: AgentPriority::Low,
            action: TaskAction::RunBash(format!(
                "time curl -s -X POST {}/api/generate \
                 -H 'Content-Type: application/json' \
                 -d '{{\"model\":\"{}\",\"prompt\":\"Say hi in 5 words.\",\"stream\":false}}' \
                 2>/dev/null | python3 -c \
                 \"import json,sys; d=json.load(sys.stdin); \
                 print(f'Tokens/s: {{d.get(\\\"eval_count\\\",0)/max(d.get(\\\"eval_duration\\\",1)/1e9,0.001):.1f}}')\"",
                config.ollama_url,
                config.primary_model
            )),
        });

        Ok(tasks)
    }
}
