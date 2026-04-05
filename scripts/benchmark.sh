#!/usr/bin/env bash
# SOVEREIGN Benchmark Script
# Tests and scores installed Ollama models
# Inspired by Project NOMAD's benchmark_service.ts
# Usage: ./scripts/benchmark.sh
# -------------------------------------------------------

OLLAMA_URL="http://localhost:11434"
PROMPT="Explain recursion in programming in exactly 50 words."

echo ""
echo "🏛️  SOVEREIGN Model Benchmark"
echo "════════════════════════════════"

# Get installed models
MODELS=$(curl -s "$OLLAMA_URL/api/tags" | python3 -c "
import json, sys
d = json.load(sys.stdin)
for m in d.get('models', []):
    print(m['name'])
" 2>/dev/null)

if [ -z "$MODELS" ]; then
  echo "No models found. Is Ollama running?"
  echo "Start with: docker compose up -d"
  exit 1
fi

echo ""
printf "%-30s %10s %12s\n" "Model" "Tok/s" "TTFT(s)"
echo "────────────────────────────────────────────────"

while IFS= read -r model; do
  START=$(date +%s%N)
  RESULT=$(curl -s -X POST "$OLLAMA_URL/api/generate" \
    -H "Content-Type: application/json" \
    -d "{\"model\":\"$model\",\"prompt\":\"$PROMPT\",\"stream\":false}" 2>/dev/null)
  END=$(date +%s%N)

  ELAPSED=$(echo "scale=2; ($END - $START) / 1000000000" | bc 2>/dev/null || echo "?")

  STATS=$(echo "$RESULT" | python3 -c "
import json, sys
try:
    d = json.load(sys.stdin)
    count = d.get('eval_count', 0)
    dur = d.get('eval_duration', 1) / 1e9
    ttft = d.get('prompt_eval_duration', 0) / 1e9
    tps = count / max(dur, 0.001)
    print(f'{tps:.1f} {ttft:.2f}')
except:
    print('? ?')
" 2>/dev/null)

  TPS=$(echo "$STATS" | cut -d' ' -f1)
  TTFT=$(echo "$STATS" | cut -d' ' -f2)

  printf "%-30s %10s %12s\n" "$model" "${TPS} tok/s" "${TTFT}s"
done <<< "$MODELS"

echo ""
echo "Results saved to ~/.sovereign/memory/world/model-benchmarks.md"
echo ""
