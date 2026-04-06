/// GraphRAG-lite — Inspired by MiroShark/MiroFish
/// Builds a JSON-based knowledge graph for long-term structured memory.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnowledgeGraph {
    pub nodes: HashMap<String, Node>,
    pub relations: Vec<Relation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub properties: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relation {
    pub source: String,
    pub target: String,
    pub type_name: String,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            relations: Vec::new(),
        }
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let graph: KnowledgeGraph = serde_json::from_str(&contents)?;
        Ok(graph)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Add a fact to the graph (simplified)
    pub fn add_fact(&mut self, source: &str, relation: &str, target: &str) {
        if !self.nodes.contains_key(source) {
            self.nodes.insert(source.to_string(), Node { id: source.to_string(), label: "Entity".to_string(), properties: HashMap::new() });
        }
        if !self.nodes.contains_key(target) {
            self.nodes.insert(target.to_string(), Node { id: target.to_string(), label: "Entity".to_string(), properties: HashMap::new() });
        }
        self.relations.push(Relation {
            source: source.to_string(),
            target: target.to_string(),
            type_name: relation.to_string(),
        });
    }
}
