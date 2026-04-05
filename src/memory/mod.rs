/// Memory system — file-based persistent memory + Qdrant semantic search
/// Inspired by the memdir system from leaked Claude Code spec:
///   ~/.sovereign/memory/you/   ← personal memory
///   ~/.sovereign/memory/world/ ← world knowledge
///
/// Relevance selection: uses embedding model to find most relevant memories
/// Freshness warnings: age-based staleness caveats (from memdir spec)

pub mod memdir;
pub use memdir::MemoryDir;
