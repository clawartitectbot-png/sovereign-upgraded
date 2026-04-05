# SOVEREIGN Setup Guide
### From zero to running in under 30 minutes on your Pi 5

---

## What You'll Have When Done

A local website at `http://localhost:8080` showing your autonomous AI OS:
- Live PHANTOM TICK countdown (fires every 15 min)
- 6 agent status panels
- Real-time activity log
- Local model health check
- Memory growth tracker

---

## Step 1 — Clone the Repo

```bash
git clone https://github.com/YOUR_USERNAME/sovereign.git
cd sovereign
```

---

## Step 2 — Run the Installer (Pi 5, as root)

```bash
chmod +x scripts/install.sh
sudo ./scripts/install.sh
```

This automatically installs:
- Docker + Docker Compose
- Rust toolchain
- Ollama (local LLM server)
- Qdrant (vector database for memory search)
- Pulls your first AI model (mistral-nemo, ~4GB)
- Creates `~/.sovereign/memory/` structure
- Builds the Rust binary

**Takes about 15–30 minutes** depending on your internet speed (model download).

---

## Step 3 — Configure Your Goals

```bash
cp config/sovereign.toml.example config/sovereign.toml
nano config/sovereign.toml
```

The important settings to change:
```toml
tick_interval_minutes = 15   # How often PHANTOM TICK fires
dream_hour = 2               # What hour Dream Engine runs (0-23)
```

Then open `~/.sovereign/memory/you/goals.md` and write your actual goals:
```bash
nano ~/.sovereign/memory/you/goals.md
```

---

## Step 4 — Start SOVEREIGN

```bash
chmod +x scripts/start.sh
./scripts/start.sh
```

---

## Step 5 — Open the Dashboard

Open your browser and go to:
```
http://localhost:8080
```

From another device on your network:
```
http://YOUR_PI_IP:8080
```

Find your Pi's IP with: `hostname -I | awk '{print $1}'`

---

## Step 6 — Pull More Models (Optional)

SOVEREIGN works best with all 4 models. Pull them in background:

```bash
# Reasoning model (~5GB) — for complex agent decisions
docker exec sovereign_ollama ollama pull deepseek-r1:7b

# Coder model (~5GB) — for CodeAgent tasks
docker exec sovereign_ollama ollama pull qwen2.5-coder:7b

# Memory embeddings (~300MB) — for semantic memory search
docker exec sovereign_ollama ollama pull nomic-embed-text:v1.5
```

---

## Step 7 — Run a Benchmark

See how fast your Pi runs each model:

```bash
chmod +x scripts/benchmark.sh
./scripts/benchmark.sh
```

Results are saved to `~/.sovereign/memory/world/model-benchmarks.md`

---

## Daily Use

SOVEREIGN runs in the background automatically. Your daily routine:

| Time | What Happens |
|------|-------------|
| Every 15 min | PHANTOM TICK fires — agents check for tasks |
| 08:00 | Morning briefing logged |
| 12:00 | Security scan |
| 02:00 | Dream Engine runs — consolidates day into memory |

**You don't have to do anything.** Just check the dashboard occasionally.

---

## Access Dashboard From Phone / Other Devices

1. Find your Pi's IP: `hostname -I | awk '{print $1}'`
2. On any device on your WiFi, go to: `http://PI_IP_HERE:8080`
3. Bookmark it on your phone for quick access

---

## Troubleshooting

**SOVEREIGN won't start:**
```bash
cargo build --release   # Rebuild the binary
./scripts/start.sh
```

**Ollama not running:**
```bash
docker compose up -d
docker compose logs -f ollama
```

**Models missing:**
```bash
docker exec sovereign_ollama ollama list
docker exec sovereign_ollama ollama pull mistral-nemo
```

**Dashboard not loading:**
```bash
# Check port is open
curl http://localhost:8080
# Make sure you're running from the project directory
cd /path/to/sovereign && ./scripts/start.sh
```

---

## File Locations

| What | Where |
|------|-------|
| Your goals | `~/.sovereign/memory/you/goals.md` |
| Daily logs | `~/.sovereign/logs/YYYY-MM-DD.md` |
| Dream output | `~/.sovereign/memory/you/daily-consolidations.md` |
| Income log | `~/.sovereign/memory/you/income-history.md` |
| Security log | `~/.sovereign/memory/world/threat-landscape.md` |
| Model benchmarks | `~/.sovereign/memory/world/model-benchmarks.md` |
| Config | `./config/sovereign.toml` |

---

*Built for Raspberry Pi 5 · 16GB RAM · Debian · Rust · Ollama · Qdrant*
