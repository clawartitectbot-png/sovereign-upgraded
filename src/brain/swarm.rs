/// Swarm Intelligence — Optimized for Context Efficiency and "Anti-Slop"
use anyhow::Result;
use crate::brain::{Brain, BrainFactory};
use crate::config::SovereignConfig;

pub struct SwarmEngine {
    config: SovereignConfig,
}

impl SwarmEngine {
    pub fn new(config: SovereignConfig) -> Self {
        Self { config }
    }

    /// Simulate a swarm debate without bloating the context window.
    pub async fn simulate_swarm(&self, problem: &str, agent_count: usize) -> Result<String> {
        let brain = BrainFactory::get_brain(&self.config);
        
        // Anti-Slop Safeguard: Trim the input problem if it's too large
        let trimmed_problem = if problem.len() > 4000 {
            format!("{}... [TRIMMED FOR EFFICIENCY]", &problem[..4000])
        } else {
            problem.to_string()
        };

        // Structured Prompt: Prevents "Slop" by forcing a specific output format
        let prompt = format!(
            "CONTEXT: {}\n\n\
             TASK: Simulate a high-speed debate between {} specialized virtual agents.\n\
             RULES: \n\
             1. No repetitive 'I agree' statements.\n\
             2. Each agent must present ONE unique counter-point.\n\
             3. Deliver only the final consensus and the single most critical risk found.\n\n\
             DEBATE START:",
            trimmed_problem, agent_count
        );

        brain.generate(&prompt, Some("You are the SOVEREIGN Swarm Orchestrator. Be concise, technical, and objective.")).await
    }
}
