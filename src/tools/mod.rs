/// Tools — Bash, file, web search
/// Inspired by Claude Code's 40+ tool registry pattern from claurst spec §tools

pub mod bash;
pub mod file;
pub mod web;

pub use bash::BashTool;
pub use file::FileTool;
pub use web::WebTool;
