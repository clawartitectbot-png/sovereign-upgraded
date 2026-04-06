pub mod client;
pub mod gemini;

use anyhow::Result;
use async_trait::async_trait;
use crate::config::SovereignConfig;

#[async_trait]
pub trait Brain: Send + Sync {
    async fn generate(&self, prompt: &str, system: Option<&str>) -> Result<String>;
}

pub struct BrainFactory;

impl BrainFactory {
    pub fn get_brain(config: &SovereignConfig) -> Box<dyn Brain> {
        match config.provider.as_str() {
            "gemini" => {
                if let Some(key) = &config.api_key {
                    Box::new(gemini::GeminiProvider::new(key, &config.primary_model))
                } else {
                    tracing::warn!("Gemini provider selected but no API key found. Falling back to Ollama.");
                    Box::new(client::OllamaClient::new(&config.ollama_url, &config.primary_model))
                }
            }
            "none" => Box::new(NoBrain),
            _ => Box::new(client::OllamaClient::new(&config.ollama_url, &config.primary_model)),
        }
    }
}

pub struct NoBrain;

#[async_trait]
impl Brain for NoBrain {
    async fn generate(&self, _prompt: &str, _system: Option<&str>) -> Result<String> {
        Ok("Sovereign running in No-AI mode. Please configure an API key or Ollama.".to_string())
    }
}
