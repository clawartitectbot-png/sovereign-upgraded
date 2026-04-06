use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use super::Brain;

pub struct GeminiProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiProvider {
    pub fn new(api_key: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            model: if model.is_empty() { "gemini-1.5-flash".to_string() } else { model.to_string() },
        }
    }
}

#[async_trait]
impl Brain for GeminiProvider {
    async fn generate(&self, prompt: &str, system: Option<&str>) -> Result<String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let system_instruction = if let Some(sys) = system {
            json!({ "parts": [{ "text": sys }] })
        } else {
            json!(null)
        };

        let body = json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }],
            "system_instruction": system_instruction,
            "generationConfig": {
                "maxOutputTokens": 2048,
            }
        });

        let response = self.client.post(&url).json(&body).send().await?;
        let json: serde_json::Value = response.json().await?;

        let text = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("Error: Empty response from Gemini")
            .to_string();

        Ok(text)
    }
}
