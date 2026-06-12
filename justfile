# =========================================
# LKProfessionals Rust Development Commands
# =========================================

set shell := ["bash", "-cu"]
set dotenv-load := true

# -----------------------------------------
# Default Command
# -----------------------------------------
default:
    @just help

# -----------------------------------------
# Development Server
# Prepares database, runs migrations, then
# starts Tailwind + Rust watcher.
# -----------------------------------------
dev:
    @echo "🚀 Starting LKProfessionals development environment..."
    @just db-ready
    npx concurrently --kill-others-on-fail \
        "npx @tailwindcss/cli -i ./assets/css/input.css -o ./static/css/app.css --watch" \
        "cargo watch -w src -w templates -w assets -w migrations -w Cargo.toml -w Cargo.lock -w build.rs -s 'cargo run'"

# -----------------------------------------
# Rust Watch Only
# Prepares database before running app.
# -----------------------------------------
watch:
    @echo "👀 Watching Rust + templates..."
    @just db-ready
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
# Prepares database before running app.
# -----------------------------------------
run:
    @echo "🦀 Running application..."
    @just db-ready
    cargo run

# -----------------------------------------
# Production Build
# -----------------------------------------
build:
    @echo "📦 Building production application..."
    @just css-build
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
# Note: current project still has warning cleanup pending.
# Use `just check` during active development.
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
# Does not touch database.
# -----------------------------------------
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    rm -rf static/css/app.css

# -----------------------------------------
# Full Clean
# Removes node_modules too.
# Use only when frontend dependencies need reinstalling.
# -----------------------------------------
clean-all:
    @echo "🧹 Deep cleaning build artifacts and frontend dependencies..."
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
    @just kill
    @just dev

# -----------------------------------------
# Database Ready Check
# Safe preflight for local development.
# Creates DB if missing and runs migrations.
# -----------------------------------------
db-ready:
    @echo "🧩 Preparing development database..."
    ./scripts/dev/ensure-db.sh

# -----------------------------------------
# Database Migrations
# Manual utility only.
# Normal development should use:
# just dev
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
# Dangerous Local Database Reset
# Guarded on purpose.
#
# Usage:
# just db-reset-local RESET_LKP_RUST
# -----------------------------------------
db-reset-local confirm:
    @if [ "{{confirm}}" != "RESET_LKP_RUST" ]; then \
        echo "❌ Refusing to reset database."; \
        echo "Usage: just db-reset-local RESET_LKP_RUST"; \
        exit 1; \
    fi
    @echo "⚠️ Dropping and recreating local development database..."
    sqlx database drop -y || true
    ./scripts/dev/ensure-db.sh

# -----------------------------------------
# Database Repair
# Safe repair without dropping data.
# Creates missing DB and runs migrations.
# -----------------------------------------
db-repair:
    @echo "🛠 Repairing local development database..."
    ./scripts/dev/ensure-db.sh

# -----------------------------------------
# Development Doctor
# Checks required tools.
# -----------------------------------------
doctor:
    @echo "🩺 Checking development tools..."
    @command -v cargo >/dev/null || { echo "❌ cargo is missing"; exit 1; }
    @command -v sqlx >/dev/null || { echo "❌ sqlx-cli is missing. Run: cargo install sqlx-cli --no-default-features --features postgres,rustls"; exit 1; }
    @command -v pg_isready >/dev/null || { echo "❌ pg_isready is missing. Install PostgreSQL client tools: sudo apt install postgresql-client"; exit 1; }
    @command -v createdb >/dev/null || { echo "❌ createdb is missing. Install PostgreSQL client tools: sudo apt install postgresql-client"; exit 1; }
    @command -v npm >/dev/null || { echo "❌ npm is missing"; exit 1; }
    @command -v npx >/dev/null || { echo "❌ npx is missing"; exit 1; }
    @command -v cargo-watch >/dev/null || { echo "❌ cargo-watch is missing. Run: cargo install cargo-watch"; exit 1; }
    @command -v concurrently >/dev/null || echo "ℹ concurrently may be installed through npm/node_modules. If dev fails, run: npm install"
    @echo "✅ Development environment looks ready."

# -----------------------------------------
# Project Verification
# Good command before commits.
# -----------------------------------------
verify:
    @echo "✅ Verifying project..."
    @just fmt
    @just check
    @just test

# -----------------------------------------
# Show Available Commands
# -----------------------------------------
help:
    @just --list