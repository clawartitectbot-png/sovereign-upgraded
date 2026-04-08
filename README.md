# ⚡ SOVEREIGN — Local Autonomous AI Assistant

**SOVEREIGN** is a lightweight, background assistant that manages your life, code, and memory locally. Built in Rust for maximum speed and privacy, it is designed to be "Broke-Friendly"—running on anything from an old laptop to a high-end workstation.

---

## 🚀 Key Features

### 🧠 **Multi-Brain Architecture**
Don't have a high-end GPU? No problem.
- **Broke-Mode:** Connects to **Google Gemini (Free Tier)** for cloud-powered intelligence with 0 local RAM usage.
- **Privacy-Mode:** Connects to **Ollama** for 100% offline, local execution.
- **Customizable:** Easily switch providers in your `config.toml`.

### 💤 **The Dream Engine (Memory)**
Inspired by **Mem0** and **MiroFish**.
- **Daily Consolidation:** Every night at 2:00 AM, SOVEREIGN summarizes your activity logs into a permanent "Memory Diary."
- **Identity Store:** Remembers who you are, what you code in, and your work habits.
- **GraphRAG-lite:** Builds a structured knowledge graph of your digital life.

### 🚜 **Autonomous CodeAgent**
Your silent partner in development.
- **Git Checkpoints:** Automatically scans your `~/projects`, commits changes, and pushes to GitHub every 15 minutes. Never lose a line of code again.

### 🐝 **Swarm Intelligence**
Need a second opinion?
- **Simulated Debates:** Spawn multiple virtual agents to brainstorm complex problems and find the best consensus before you act.

---

## 🛠️ Quick Start

### 1. Requirements
- **Rust:** Install from [rustup.rs](https://rustup.rs/)
- **Provider:** A **Gemini API Key** (Free) or **Ollama** (Local).

### 2. Setup
```powershell
./setup.ps1
```

### 3. Run
```powershell
cargo run
```

### 4. Dashboard
Open your browser to: **[http://localhost:8080](http://localhost:8080)**

---

## 🧩 Skill System
Add new powers to SOVEREIGN by dropping a JSON manifest into the `skills/` folder. It's modular, extensible, and inspired by **OpenClaw** and **Claude Code**.

---

## 📜 License
Licensed under the **MIT License**. Safe, trusted, and open source.

## 🤝 Contributing
Contributions are welcome! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---
*Built with ❤️ for the Sovereign Community.*
