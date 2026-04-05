# 🏛️ SOVEREIGN
### Your Personal Autonomous AI Operating System

> Runs 100% on your Raspberry Pi 5 (16GB RAM). No cloud. No subscriptions. No one else's rules.
> Works for you 24/7 — even while you sleep.

---

## What Is SOVEREIGN?

SOVEREIGN is not a chatbot. It's an **autonomous AI agent swarm** that runs permanently on your local hardware, thinks every 15 minutes, learns about you every day, and works to achieve your goals without you having to ask.

Built on architecture reverse-engineered from the best AI coding agents in the world (Claude Code, OpenClaw, Project NOMAD), then rewritten clean-room in Rust for your Pi 5.

---

## The 7 Layers

| Layer | Name | What It Does |
|-------|------|-------------|
| 1 | **PHANTOM TICK** | Fires every 15 min — autonomous decision loop (KAIROS-inspired) |
| 2 | **SWARM COORDINATOR** | 6 parallel agents working in background |
| 3 | **LOCAL INFRA** | Ollama + Qdrant RAG + Docker (NOMAD-inspired) |
| 4 | **SKILLS + PLUGINS** | 14 skill docs + 14 official plugin patterns |
| 5 | **MODEL PIPELINE** | Local Ollama models, auto-benchmarked, auto-promoted |
| 6 | **DREAM ENGINE** | Nightly memory consolidation — gets smarter about you |
| 7 | **HARDWARE** | Raspberry Pi 5, 16GB RAM, Debian |

---

## The 6 Agents

| Agent | Role |
|-------|------|
| **CodeAgent** | Writes code, opens PRs, fixes bugs autonomously |
| **IncomeAgent** | Finds freelance work, manages gigs, tracks earnings |
| **SecurityAgent** | Monitors network, patches vulns, weekly threat reports |
| **LearningAgent** | Reads papers/news, fills knowledge gaps, builds your personal KB |
| **FinanceAgent** | Tracks income/expenses, tax prep, budget alerts |
| **UpgradeAgent** | Benchmarks Ollama models, promotes the best one automatically |

---

## Quick Start (Pi 5)

```bash
# 1. Clone this repo
git clone https://github.com/YOUR_USERNAME/sovereign.git
cd sovereign

# 2. Run the install script (sets up Docker, Ollama, Qdrant)
chmod +x scripts/install.sh
sudo ./scripts/install.sh

# 3. Configure your goals
cp config/sovereign.toml.example config/sovereign.toml
nano config/sovereign.toml

# 4. Start SOVEREIGN
./scripts/start.sh

# 5. Open the dashboard
# Go to http://localhost:8080 in your browser
```

---

## Local Dashboard

Once running, open your browser to `http://localhost:8080` (or `http://PI_IP:8080` from another device on your network).

The dashboard shows:
- Live PHANTOM TICK status
- Agent activity feeds
- Memory growth over time
- Model benchmark scores
- Income/finance summary

---

## Architecture Sources

This project is a **clean-room implementation** inspired by:
- `leaked-claude-code` — KAIROS tick, Dream Engine, multi-agent orchestration patterns
- `claurst` — Rust reimplementation specs (14 spec docs, 990KB of architectural detail)
- `claw-code` — Rust agent harness, provider abstraction, LSP client
- `project-nomad` — Ollama management, Qdrant RAG, Docker orchestration, benchmarking
- `clear-code` — Skill patterns, tool architecture conventions
- `claude-code` (official) — Plugin system, hooks, devcontainer setup

No proprietary source code is used. All implementation is original Rust built from behavioral specs.

---

## Hardware Requirements

**Minimum (barebones):**
- Raspberry Pi 5, 8GB RAM
- 32GB microSD or USB SSD
- Debian/Ubuntu

**Recommended (full stack):**
- Raspberry Pi 5, **16GB RAM** ← you have this ✅
- 128GB+ SSD (USB 3.0)
- Ethernet connection (for initial setup only)

---

## Folder Structure

```
sovereign/
├── src/
│   ├── main.rs              ← Entry point + daemon loop
│   ├── tick/                ← PHANTOM TICK (KAIROS-inspired)
│   ├── dream/               ← Dream Engine (autoDream-inspired)
│   ├── agents/              ← 6 autonomous agents
│   ├── memory/              ← File-based memory + Qdrant search
│   ├── tools/               ← Bash, file, web tools
│   └── ollama/              ← Local model client
├── web/                     ← Local dashboard (HTML/JS)
├── scripts/                 ← install.sh, start.sh, benchmark.sh
├── config/                  ← sovereign.toml config
├── docs/                    ← Architecture deep-dives
└── docker-compose.yml       ← Ollama + Qdrant + services
```

---

## License

MIT — this is yours. Build on it, modify it, run it forever.

---

*Built for: Raspberry Pi 5 16GB | Rust | Ollama | Qdrant | 100% local*
