/// FileTool — Read, write, list files
use anyhow::Result;
use std::path::Path;

pub struct FileTool;

impl FileTool {
    pub async fn read(path: &str) -> Result<String> {
        let expanded = shellexpand::tilde(path).to_string();
        Ok(tokio::fs::read_to_string(&expanded).await?)
    }

    pub async fn write(path: &str, content: &str) -> Result<()> {
        let expanded = shellexpand::tilde(path).to_string();
        if let Some(parent) = Path::new(&expanded).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        Ok(tokio::fs::write(&expanded, content).await?)
    }

    pub async fn append(path: &str, content: &str) -> Result<()> {
        let expanded = shellexpand::tilde(path).to_string();
        let existing = tokio::fs::read_to_string(&expanded).await.unwrap_or_default();
        tokio::fs::write(&expanded, format!("{}{}", existing, content)).await?;
        Ok(())
    }

    pub async fn list_dir(path: &str) -> Result<Vec<String>> {
        let expanded = shellexpand::tilde(path).to_string();
        let mut entries = tokio::fs::read_dir(&expanded).await?;
        let mut files = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            files.push(entry.file_name().to_string_lossy().to_string());
        }
        files.sort();
        Ok(files)
    }
}
