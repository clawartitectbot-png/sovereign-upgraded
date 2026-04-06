/// Skill System — Inspired by OpenClaw and Claude Code
/// Modular, extensible toolsets for the SOVEREIGN agent.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkillManifest {
    pub name: String,
    pub description: String,
    pub commands: Vec<SkillCommand>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkillCommand {
    pub name: String,
    pub description: String,
    pub shell_template: String,
}

pub struct SkillRegistry {
    pub skills_dir: PathBuf,
}

impl SkillRegistry {
    pub fn new(base_path: &str) -> Self {
        let path = PathBuf::from(shellexpand::tilde(base_path).to_string()).join("skills");
        std::fs::create_dir_all(&path).ok();
        Self { skills_dir: path }
    }

    pub fn list_skills(&self) -> Vec<SkillManifest> {
        let mut skills = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.skills_dir) {
            for entry in entries.flatten() {
                let manifest_path = entry.path().join("manifest.json");
                if manifest_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(manifest_path) {
                        if let Ok(manifest) = serde_json::from_str::<SkillManifest>(&content) {
                            skills.push(manifest);
                        }
                    }
                }
            }
        }
        skills
    }
}
