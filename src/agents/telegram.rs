/// Telegram Agent — Connects SOVEREIGN to your phone
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub struct TelegramBridge {
    client: Client,
    token: String,
    chat_id: String,
}

impl TelegramBridge {
    pub fn new(token: &str, chat_id: &str) -> Self {
        Self {
            client: Client::new(),
            token: token.to_string(),
            chat_id: chat_id.to_string(),
        }
    }

    pub async fn send_message(&self, text: &str) -> Result<()> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);
        let body = json!({
            "chat_id": self.chat_id,
            "text": text,
            "parse_mode": "Markdown"
        });

        self.client.post(&url).json(&body).send().await?;
        Ok(())
    }

    /// Check for new messages (long polling)
    pub async fn get_updates(&self) -> Result<Vec<String>> {
        let url = format!("https://api.telegram.org/bot{}/getUpdates", self.token);
        let response = self.client.get(&url).send().await?;
        let json: serde_json::Value = response.json().await?;

        let mut messages = Vec::new();
        if let Some(results) = json["result"].as_array() {
            for res in results {
                if let Some(text) = res["message"]["text"].as_str() {
                    messages.push(text.to_string());
                }
            }
        }
        Ok(messages)
    }
}
