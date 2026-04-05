# SOVEREIGN Architecture

## Sources

This is a clean-room implementation built by studying behavioral patterns from:

| Source | What We Took |
|--------|-------------|
| `leaked-claude-code` (512K lines TS) | KAIROS tick pattern, autoDream spec, memdir system, feature flags |
| `claurst` (14 spec docs, 990KB) | Complete behavioral specs for every subsystem |
| `claw-code` (Rust, 40K lines) | Provider abstraction, API client, LSP, compat-harness |
| `project-nomad` (28K lines TS) | Ollama management, Qdrant RAG, Docker orchestration, benchmark scoring |
| `clear-code` (skill docs) | Tool architecture patterns, TypeScript-to-Rust translation conventions |
| `claude-code` (official) | Plugin system, hooks spec, devcontainer, CI patterns |

No proprietary source code is copied. All Rust code is original.

---

## PHANTOM TICK (src/tick/)

**Inspired by:** KAIROS system in leaked-claude-code

From the leaked source (`proactive/index.js`):
- `feature('PROACTIVE') || feature('KAIROS')` — gated behind compile-time flag
- Injects `<tick>` prompts on a schedule
- Appends to append-only daily log files
- Fires even if user is idle

SOVEREIGN's implementation:
- Runs every 15 minutes (configurable in `sovereign.toml`)
- Polls each agent for `Vec<AgentTask>`
- Sorts by `AgentPriority` (Critical → High → Normal → Low)
- Executes Critical/High immediately, spawns Normal as background tasks
- Logs every tick to `~/.sovereign/logs/YYYY-MM-DD.md`

---

## DREAM ENGINE (src/dream/)

**Inspired by:** `autoDream.ts` in leaked-claude-code

From the spec (claurst §06_services_context_state.md):
- `SESSION_SCAN_INTERVAL_MS = 10 * 60 * 1000`
- Gate: time check (minHours) → session count → consolidation lock
- `initAutoDream()` registers as a post-sampling hook
- Spawns forked agent using `buildConsolidationPrompt()`
- Config: `tengu_onyx_plover` GrowthBook flag

SOVEREIGN's implementation:
- Checks every 30 minutes if it's 2AM
- Reads today's tick log from `~/.sovereign/logs/`
- Sends to Ollama with consolidation prompt
- Saves extracted insights to `~/.sovereign/memory/you/`

---

## MEMORY SYSTEM (src/memory/)

**Inspired by:** `memdir` system in leaked-claude-code spec §11

From the spec:
- Auto memory: `~/.claude/projects/<sanitized-git-root>/memory/`
- Memory scanning: reads frontmatter headers to build manifest
- Relevance selection: uses Sonnet model call to pick up to 5 relevant files
- Freshness warnings: age-based staleness caveats as `<system-reminder>` tags

SOVEREIGN's implementation:
- `~/.sovereign/memory/you/` — personal knowledge (goals, income, style)
- `~/.sovereign/memory/world/` — world knowledge (opportunities, threats, benchmarks)
- Reads all `.md` files with staleness warnings for files >7 days old
- Appends new content with timestamps

---

## SWARM COORDINATOR (src/agents/)

**Inspired by:** `coordinatorMode.ts` in leaked-claude-code

From the spec (claurst §06):
- `isCoordinatorMode()` — detects multi-worker mode
- `getCoordinatorUserContext()` — injects shared scratchpad
- Multiple parallel subagents with session-resume alignment

SOVEREIGN's 6 agents:
- **CodeAgent** — scans local git projects, checks CI, monitors GitHub issues
- **IncomeAgent** — monitors job boards, tracks freelance earnings
- **SecurityAgent** — system updates, port scanning, failed login detection
- **LearningAgent** — HackerNews, ArXiv, fills knowledge gaps
- **FinanceAgent** — daily P&L logging, tax prep, budget tracking
- **UpgradeAgent** — Ollama model benchmarking, auto-promotion

---

## LOCAL MODEL PIPELINE (src/ollama/)

**Inspired by:** Project NOMAD's `ollama_service.ts`

From NOMAD's source:
- Lazy initialization via `_initializeOllamaClient()`
- Progress streaming with `broadcastDownloadProgress()`
- Check if model already installed before pulling
- Model cache with 24-hour TTL

SOVEREIGN's implementation:
- `OllamaClient::generate()` — blocking completion
- `OllamaClient::embed()` — 768-dim embeddings (nomic-embed-text:v1.5)
- `OllamaClient::health_check()` — pre-tick validation
- Model routing: primary (speed) / coder / reasoning

---

## RAG MEMORY SEARCH

**Inspired by:** Project NOMAD's `rag_service.ts`

From NOMAD's source:
- `EMBEDDING_MODEL = 'nomic-embed-text:v1.5'`
- `EMBEDDING_DIMENSION = 768`
- `TARGET_TOKENS_PER_CHUNK = 1700`
- `EMBEDDING_BATCH_SIZE = 8` (conservative for low-end hardware)
- Cosine similarity search in Qdrant

SOVEREIGN's use:
- Memory files are chunked and embedded nightly by Dream Engine
- On each tick, agents query Qdrant for relevant past context
- Top-5 chunks injected into agent prompts

---

## BENCHMARK SYSTEM

**Inspired by:** Project NOMAD's `benchmark_service.ts`

From NOMAD's source:
```
SCORE_WEIGHTS = {
  ai_tokens_per_second: 0.30,
  cpu: 0.25,
  memory: 0.15,
  ai_ttft: 0.10,
  disk_read: 0.10,
  disk_write: 0.10,
}
AI_BENCHMARK_MODEL = 'llama3.2:1b'
AI_BENCHMARK_PROMPT = 'Explain recursion in programming in exactly 100 words.'
```

SOVEREIGN's `scripts/benchmark.sh` runs the same pattern and logs to memory.

---

## DASHBOARD (web/)

Simple HTML/CSS/JS single-file dashboard:
- PHANTOM TICK live countdown with progress bar
- Agent status grid (6 agents)
- Ollama model health (polling `localhost:11434/api/tags`)
- Live activity log (WebSocket-ready)
- Memory file count

Served by Axum (`src/web/mod.rs`) at `http://localhost:8080`
