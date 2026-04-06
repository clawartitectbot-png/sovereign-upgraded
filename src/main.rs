use anyhow::Result;
use clap::Parser;
use tracing::{info, warn};

mod agents;
mod dream;
mod memory;
mod brain;
mod tick;
mod tools;
mod web;

use tick::PhantomTick;
use dream::DreamEngine;

#[derive(Parser)]
#[command(name = "sovereign")]
#[command(about = "Your Local Autonomous AI Personal Assistant")]
struct Args {
    #[arg(short, long, default_value = "config/sovereign.toml")]
    config: String,

    #[arg(short, long)]
    daemon: bool,

    #[arg(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("sovereign=info,warn")
        .init();

    let args = Args::parse();

    info!("⚡ SOVEREIGN starting up...");

    let config = config::load(&args.config).unwrap_or_else(|e| {
        warn!("Config not found ({}), using defaults", e);
        config::SovereignConfig::default()
    });

    info!("🧠 Provider: {} | Model: {}", config.provider, config.primary_model);

    let port = args.port;
    tokio::spawn(async move {
        web::serve(port).await.expect("Dashboard failed");
    });

    let dream = DreamEngine::new(config.clone());
    tokio::spawn(async move {
        dream.run().await;
    });

    let tick = PhantomTick::new(config.clone());
    info!("👁️  PHANTOM TICK armed — firing every {} minutes", config.tick_interval_minutes);
    tick.run().await?;

    Ok(())
}

pub mod config {
    use serde::{Deserialize, Serialize};
    use anyhow::Result;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SovereignConfig {
        pub tick_interval_minutes: u64,
        pub memory_path: String,
        pub provider: String,
        pub api_key: Option<String>,
        pub ollama_url: String,
        pub primary_model: String,
        pub dream_hour: u32,
        pub agents: AgentConfig,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AgentConfig {
        pub code_agent: bool,
        pub income_agent: bool,
        pub security_agent: bool,
        pub learning_agent: bool,
        pub finance_agent: bool,
        pub upgrade_agent: bool,
    }

    impl Default for SovereignConfig {
        fn default() -> Self {
            Self {
                tick_interval_minutes: 15,
                memory_path: "~/.sovereign/memory".to_string(),
                provider: "ollama".to_string(),
                api_key: None,
                ollama_url: "http://localhost:11434".to_string(),
                primary_model: "mistral-nemo:latest".to_string(),
                dream_hour: 2,
                agents: AgentConfig {
                    code_agent: true,
                    income_agent: true,
                    security_agent: true,
                    learning_agent: true,
                    finance_agent: true,
                    upgrade_agent: true,
                },
            }
        }
    }

    pub fn load(path: &str) -> Result<SovereignConfig> {
        let contents = std::fs::read_to_string(path)?;
        let config: SovereignConfig = toml::from_str(&contents)?;
        Ok(config)
    }
}
