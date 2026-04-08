# SOVEREIGN One-Click Setup for Windows
Write-Host "⚡ Setting up SOVEREIGN Local Assistant..." -ForegroundColor Emerald

# Check for Rust
if (!(Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust not found. Please install it from https://rustup.rs/" -ForegroundColor Red
    exit
}

# Create memory folders
$memPath = [System.IO.Path]::Combine($env:USERPROFILE, ".sovereign", "memory")
if (!(Test-Path $memPath)) {
    New-Item -ItemType Directory -Force -Path $memPath | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $memPath "you") | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $memPath "world") | Out-Null
    Write-Host "✅ Created memory directories at $memPath" -ForegroundColor Cyan
}

# Check for config
if (!(Test-Path "config/sovereign.toml")) {
    Copy-Item "config/sovereign.toml.example" "config/sovereign.toml" -ErrorAction SilentlyContinue
    Write-Host "✅ Created default config/sovereign.toml" -ForegroundColor Cyan
}

Write-Host "🚀 Setup complete! Run 'cargo run' to start SOVEREIGN." -ForegroundColor Emerald
