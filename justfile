# =========================================
# LKProfessionals Rust Development Commands
# =========================================

set shell := ["bash", "-cu"]
set dotenv-load := true

# -----------------------------------------
# Development Server
# Rust app handles migrations + seeds
# -----------------------------------------
dev:
    @echo "🚀 Starting LKProfessionals development environment..."
    npx concurrently --kill-others-on-fail \
        "npx @tailwindcss/cli -i ./assets/css/input.css -o ./static/css/app.css --watch" \
        "cargo watch -w src -w templates -w assets -w migrations -w Cargo.toml -w Cargo.lock -w build.rs -s 'cargo run'"

# -----------------------------------------
# Rust Watch Only
# Rust app handles migrations + seeds
# -----------------------------------------
watch:
    @echo "👀 Watching Rust + templates..."
    cargo watch -w src -w templates -w assets -w migrations -w Cargo.toml -w Cargo.lock -w build.rs -s 'cargo run'

# -----------------------------------------
# Tailwind Watch Only
# -----------------------------------------
css:
    @echo "🎨 Watching Tailwind CSS..."
    npx @tailwindcss/cli \
        -i ./assets/css/input.css \
        -o ./static/css/app.css \
        --watch

# -----------------------------------------
# Production CSS Build
# -----------------------------------------
css-build:
    @echo "🎨 Building production CSS..."
    npx @tailwindcss/cli \
        -i ./assets/css/input.css \
        -o ./static/css/app.css \
        --minify

# -----------------------------------------
# Development Run Without Watch
# Rust app handles migrations + seeds
# -----------------------------------------
run:
    @echo "🦀 Running application..."
    cargo run

# -----------------------------------------
# Production Build
# -----------------------------------------
build:
    @echo "📦 Building production application..."
    cargo build --release

# -----------------------------------------
# Install Dependencies
# -----------------------------------------
install:
    @echo "📥 Installing frontend dependencies..."
    npm install

# -----------------------------------------
# Cargo Check
# -----------------------------------------
check:
    @echo "🔍 Checking project..."
    cargo check

# -----------------------------------------
# Formatting
# -----------------------------------------
fmt:
    @echo "✨ Formatting Rust code..."
    cargo fmt

# -----------------------------------------
# Clippy Linting
# -----------------------------------------
lint:
    @echo "🛡 Running Clippy..."
    cargo clippy -- -D warnings

# -----------------------------------------
# Tests
# -----------------------------------------
test:
    @echo "🧪 Running tests..."
    cargo test

# -----------------------------------------
# Clean Build Files
# -----------------------------------------
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    rm -rf node_modules
    rm -rf static/css/app.css

# -----------------------------------------
# Reset Port 3000
# -----------------------------------------
kill:
    @echo "💀 Killing port 3000..."
    sudo fuser -k 3000/tcp || true

# -----------------------------------------
# Full Restart
# -----------------------------------------
restart:
    @echo "♻ Restarting development environment..."
    just kill
    just dev

# -----------------------------------------
# Database Migrations
# Manual utility only.
# Normal dev should use: just dev
# -----------------------------------------
migrate:
    @echo "🗄 Running database migrations..."
    sqlx migrate run

# -----------------------------------------
# Migration Status
# -----------------------------------------
migrate-status:
    @echo "📋 Showing migration status..."
    sqlx migrate info

# -----------------------------------------
# Create Migration
# Usage:
# just migration create_contact_messages
# -----------------------------------------
migration name:
    @echo "📝 Creating migration {{name}}..."
    sqlx migrate add {{name}}

# -----------------------------------------
# Development Doctor
# -----------------------------------------
doctor:
    @echo "🩺 Checking development tools..."
    @command -v cargo >/dev/null || { echo "❌ cargo is missing"; exit 1; }
    @command -v sqlx >/dev/null || { echo "❌ sqlx-cli is missing. Run: cargo install sqlx-cli --no-default-features --features postgres"; exit 1; }
    @command -v psql >/dev/null || { echo "❌ psql is missing. Install PostgreSQL client tools."; exit 1; }
    @command -v npm >/dev/null || { echo "❌ npm is missing"; exit 1; }
    @command -v npx >/dev/null || { echo "❌ npx is missing"; exit 1; }
    @echo "✅ Development environment looks ready."

# -----------------------------------------
# Show Available Commands
# -----------------------------------------
help:
    @just --list