# Changelog

All notable changes to the SOVEREIGN project will be documented in this file.

## [1.1.0] - 2026-04-06
### Added
- **Multi-Brain Architecture**: Support for both local (Ollama) and cloud (Gemini) AI providers.
- **Broke-Friendly Mode**: Optimized for low-RAM devices using the Gemini Free Tier API.
- **Autonomous CodeAgent**: Real-world logic for auto-committing and pushing local changes to GitHub.
- **Swarm Intelligence**: New `SwarmEngine` for multi-agent "simulated debates" to reduce AI slop.
- **Mem0 Integration**: Personalized memory store for long-term user preference tracking.
- **GraphRAG-lite**: Structured knowledge graph memory built from daily logs.
- **Telegram Bridge**: Optional module for mobile control and notifications.
- **Web Dashboard**: Modern emerald-themed UI for mission control.
- **Anti-Slop Logic**: Strict prompt engineering and context pruning for higher quality output.

### Changed
- Refactored `src/ollama` to `src/brain` for multi-provider support.
- Updated project terminology from "AI OS" to "Local Autonomous Assistant."

### Security
- MIT License added.
- Full security audit performed; no secrets or keys found in codebase.
