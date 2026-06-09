# =========================================
# LKProfessionals Rust Development Commands
# =========================================

set shell := ["bash", "-cu"]

# -----------------------------------------
# Development Server
# -----------------------------------------
dev:
    @echo "🚀 Starting LKProfessionals development environment..."
    npx concurrently --kill-others-on-fail \
        "npx @tailwindcss/cli -i ./assets/css/input.css -o ./static/css/app.css --watch" \
        "cargo watch -w src -w templates -w assets -w Cargo.toml -w Cargo.lock -s 'cargo run'"

# -----------------------------------------
# Rust Watch Only
# -----------------------------------------
watch:
    @echo "👀 Watching Rust + templates..."
    cargo watch -w src -w templates -w assets -w Cargo.toml -w Cargo.lock -s 'cargo run'

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
# -----------------------------------------
migrate:
    @echo "🗄 Running database migrations..."
    sqlx migrate run

# -----------------------------------------
# Create Migration
# -----------------------------------------
migration name:
    @echo "📝 Creating migration {{name}}..."
    sqlx migrate add {{name}}

# -----------------------------------------
# Show Available Commands
# -----------------------------------------
help:
    @just --list