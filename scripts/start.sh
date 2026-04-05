#!/usr/bin/env bash
# SOVEREIGN Start Script
# Usage: ./scripts/start.sh
# -------------------------------------------------------

set -e
CYAN='\033[0;36m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'

echo ""
echo -e "${CYAN}🏛️  Starting SOVEREIGN...${NC}"

# Check Docker services
if docker compose ps | grep -q "running"; then
  echo -e "${GREEN}[✓]${NC} Docker services running"
else
  echo -e "${YELLOW}[!]${NC} Starting Docker services..."
  docker compose up -d
  sleep 3
fi

# Check Ollama
if curl -s http://localhost:11434/api/tags >/dev/null 2>&1; then
  MODELS=$(curl -s http://localhost:11434/api/tags | python3 -c "import json,sys; d=json.load(sys.stdin); print(len(d.get('models',[])))" 2>/dev/null || echo "?")
  echo -e "${GREEN}[✓]${NC} Ollama running ($MODELS models installed)"
else
  echo -e "${YELLOW}[!]${NC} Ollama not responding yet — it may still be starting"
fi

# Check Qdrant
if curl -s http://localhost:6333/readyz >/dev/null 2>&1; then
  echo -e "${GREEN}[✓]${NC} Qdrant running"
else
  echo -e "${YELLOW}[!]${NC} Qdrant not responding yet"
fi

# Start SOVEREIGN binary
echo -e "${CYAN}[SOVEREIGN]${NC} Launching daemon..."
echo ""

./target/release/sovereign --config config/sovereign.toml "$@"
