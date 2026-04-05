/// Ollama Client — Local model inference
///
/// Inspired by Project NOMAD's ollama_service.ts and claw-code's provider abstraction.
/// Connects to local Ollama instance, streams responses, handles RAG injection.

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GenerateResponse {
    pub response: String,
    pub done: bool,
    pub eval_count: Option<u64>,
    pub eval_duration: Option<u64>,
}

#[derive(Serialize)]
struct EmbedRequest {
    model: String,
    prompt: String,
}

#[derive(Deserialize)]
pub struct EmbedResponse {
    pub embedding: Vec<f32>,
}

#[derive(Deserialize, Debug)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Generate a completion from the model
    pub async fn generate(&self, model: &str, prompt: &str, system: Option<&str>) -> Result<String> {
        let req = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            system: system.map(|s| s.to_string()),
        };

        let response = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&req)
            .send()
            .await?
            .json::<GenerateResponse>()
            .await?;

        if let (Some(count), Some(duration)) = (response.eval_count, response.eval_duration) {
            let tps = count as f64 / (duration as f64 / 1e9);
            info!("  Ollama [{model}]: {count} tokens @ {tps:.1} tok/s");
        }

        Ok(response.response)
    }

    /// Get embeddings for RAG (using nomic-embed-text)
    pub async fn embed(&self, model: &str, text: &str) -> Result<Vec<f32>> {
        let req = EmbedRequest {
            model: model.to_string(),
            prompt: text.to_string(),
        };

        let response = self.client
            .post(format!("{}/api/embeddings", self.base_url))
            .json(&req)
            .send()
            .await?
            .json::<EmbedResponse>()
            .await?;

        Ok(response.embedding)
    }

    /// List installed models
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let response = self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?
            .json::<TagsResponse>()
            .await?;

        Ok(response.models)
    }

    /// Check if Ollama is reachable
    pub async fn health_check(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    /// Pull a model from Ollama registry
    pub async fn pull_model(&self, model: &str) -> Result<()> {
        #[derive(Serialize)]
        struct PullRequest { name: String, stream: bool }

        info!("Pulling model: {}", model);
        self.client
            .post(format!("{}/api/pull", self.base_url))
            .json(&PullRequest { name: model.to_string(), stream: false })
            .send()
            .await?;

        info!("Model pulled: {}", model);
        Ok(())
    }
}
