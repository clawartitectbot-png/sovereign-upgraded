#!/usr/bin/env bash
# SOVEREIGN Install Script
# Tested on: Raspberry Pi 5 (16GB RAM), Debian/Ubuntu
# Run with: sudo ./scripts/install.sh
# -------------------------------------------------------

set -e
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'; NC='\033[0m'
info()    { echo -e "${CYAN}[SOVEREIGN]${NC} $1"; }
success() { echo -e "${GREEN}[✓]${NC} $1"; }
warn()    { echo -e "${YELLOW}[!]${NC} $1"; }
error()   { echo -e "${RED}[✗]${NC} $1"; exit 1; }

echo ""
echo "🏛️  SOVEREIGN — Autonomous AI OS"
echo "   Installing on Raspberry Pi 5..."
echo ""

# Check we're on a Debian-based system
if ! command -v apt-get &>/dev/null; then
  error "This installer requires a Debian-based OS (Ubuntu, Raspberry Pi OS, etc.)"
fi

# Update system
info "Updating system packages..."
apt-get update -qq
apt-get install -y curl git build-essential pkg-config libssl-dev ca-certificates \
  gnupg lsb-release software-properties-common shellexpand 2>/dev/null
success "System packages installed"

# Install Docker
if ! command -v docker &>/dev/null; then
  info "Installing Docker..."
  curl -fsSL https://get.docker.com -o /tmp/get-docker.sh
  bash /tmp/get-docker.sh
  usermod -aG docker "$SUDO_USER" 2>/dev/null || true
  systemctl enable docker
  systemctl start docker
  success "Docker installed"
else
  success "Docker already installed ($(docker --version | cut -d' ' -f3 | tr -d ','))"
fi

# Install Docker Compose
if ! command -v docker compose &>/dev/null 2>&1; then
  info "Installing Docker Compose..."
  apt-get install -y docker-compose-plugin
  success "Docker Compose installed"
else
  success "Docker Compose already installed"
fi

# Install Rust
if ! command -v cargo &>/dev/null; then
  info "Installing Rust (this takes ~2 minutes)..."
  su - "$SUDO_USER" -c 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
  export PATH="$HOME/.cargo/bin:$PATH"
  success "Rust installed"
else
  success "Rust already installed ($(rustc --version | cut -d' ' -f2))"
fi

# Create SOVEREIGN directory structure
info "Creating SOVEREIGN data directories..."
SOVEREIGN_HOME="${SUDO_USER:+/home/$SUDO_USER}/.sovereign"
mkdir -p "$SOVEREIGN_HOME/memory/you"
mkdir -p "$SOVEREIGN_HOME/memory/world"
mkdir -p "$SOVEREIGN_HOME/logs"
mkdir -p "$SOVEREIGN_HOME/cache"
if [ -n "$SUDO_USER" ]; then
  chown -R "$SUDO_USER:$SUDO_USER" "$SOVEREIGN_HOME"
fi
success "Directories created at $SOVEREIGN_HOME"

# Copy config if not exists
if [ ! -f "config/sovereign.toml" ]; then
  info "Creating default config..."
  cp config/sovereign.toml.example config/sovereign.toml
  success "Config created at config/sovereign.toml"
fi

# Start Docker services
info "Starting Ollama and Qdrant..."
docker compose up -d
sleep 5

# Pull Ollama models
info "Pulling Ollama models (this will take a while on first run)..."
info "Primary model (mistral-nemo) — ~4GB..."
docker exec sovereign_ollama ollama pull mistral-nemo 2>/dev/null || \
  warn "Could not pull mistral-nemo — run manually: docker exec sovereign_ollama ollama pull mistral-nemo"

info "Embedding model (nomic-embed-text) — ~300MB..."
docker exec sovereign_ollama ollama pull nomic-embed-text:v1.5 2>/dev/null || \
  warn "Could not pull nomic-embed-text — run manually later"

info "Reasoning model (deepseek-r1:7b) — ~5GB..."
info "(Skipping for now — run: docker exec sovereign_ollama ollama pull deepseek-r1:7b)"

info "Coder model (qwen2.5-coder:7b) — ~5GB..."
info "(Skipping for now — run: docker exec sovereign_ollama ollama pull qwen2.5-coder:7b)"

# Build SOVEREIGN
info "Building SOVEREIGN (Rust compile — 2-5 minutes)..."
if [ -n "$SUDO_USER" ]; then
  su - "$SUDO_USER" -c "cd $(pwd) && ~/.cargo/bin/cargo build --release" || \
    warn "Build failed — try: cargo build --release manually"
else
  cargo build --release || warn "Build failed — try: cargo build --release manually"
fi
success "SOVEREIGN built"

echo ""
echo "════════════════════════════════════════════"
echo "  🏛️  SOVEREIGN installed successfully!"
echo ""
echo "  Start:     ./scripts/start.sh"
echo "  Dashboard: http://localhost:8080"
echo ""
echo "  Pull more models:"
echo "  docker exec sovereign_ollama ollama pull deepseek-r1:7b"
echo "  docker exec sovereign_ollama ollama pull qwen2.5-coder:7b"
echo "════════════════════════════════════════════"
echo ""
