/// WebTool — Search and fetch from the internet
use anyhow::Result;
use reqwest::Client;

pub struct WebTool {
    client: Client,
}

impl WebTool {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("SOVEREIGN/0.1 (personal AI agent)")
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap(),
        }
    }

    /// DuckDuckGo instant answer API (no key needed)
    pub async fn search(&self, query: &str) -> Result<String> {
        let url = format!(
            "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
            urlencoding::encode(query)
        );

        let response = self.client.get(&url).send().await?;
        let body: serde_json::Value = response.json().await?;

        let abstract_text = body["Abstract"].as_str().unwrap_or("").to_string();
        let answer = body["Answer"].as_str().unwrap_or("").to_string();

        if !answer.is_empty() {
            Ok(answer)
        } else if !abstract_text.is_empty() {
            Ok(abstract_text)
        } else {
            Ok(format!("No instant answer found for: {}", query))
        }
    }

    /// Fetch raw HTML/text from a URL
    pub async fn fetch(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        Ok(response.text().await?)
    }

    /// Fetch HackerNews top stories
    pub async fn hn_top(&self, count: usize) -> Result<Vec<serde_json::Value>> {
        let ids: Vec<u64> = self.client
            .get("https://hacker-news.firebaseio.com/v0/topstories.json")
            .send().await?
            .json().await?;

        let mut stories = Vec::new();
        for id in ids.iter().take(count) {
            let story: serde_json::Value = self.client
                .get(&format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id))
                .send().await?
                .json().await?;
            stories.push(story);
        }

        Ok(stories)
    }
}
