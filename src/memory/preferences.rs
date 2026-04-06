/// Personalized Memory (Mem0-lite) — Searchable User Identity Store
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserFact {
    pub fact: String,
    pub category: String, // e.g., "Coding", "Finance", "Personal"
    pub timestamp: DateTime<Local>,
    pub importance: u8, // 1-5
}

pub struct PreferenceStore {
    path: PathBuf,
    pub facts: Vec<UserFact>,
}

impl PreferenceStore {
    pub fn new(memory_path: &str) -> Self {
        let path = PathBuf::from(shellexpand::tilde(memory_path).to_string()).join("you").join("preferences.json");
        let facts = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };
        Self { path, facts }
    }

    pub fn add_fact(&mut self, fact: &str, category: &str, importance: u8) -> Result<()> {
        self.facts.push(UserFact {
            fact: fact.to_string(),
            category: category.to_string(),
            timestamp: Local::now(),
            importance,
        });
        self.save()
    }

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.facts)?;
        std::fs::write(&self.path, json)?;
        Ok(())
    }

    /// Return all facts as a context string for the AI Agent
    pub fn to_context(&self) -> String {
        self.facts.iter()
            .map(|f| format!("[{}] (Imp: {}): {}", f.category, f.importance, f.fact))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
