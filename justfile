# Development server with auto-reload on file changes
dev:
    @echo "Starting development server with auto-reload..."
    npx concurrently --kill-others-on-fail \
        "npx @tailwindcss/cli -i ./assets/css/input.css -o ./static/css/app.css --watch" \
        "cargo watch --no-process-group --watch src --watch templates --delay 0.5 -x run"

# Watch Rust and template files, reload on change
watch:
    @echo "Watching for changes in src/ and templates..."
    cargo watch --no-process-group --watch src --watch templates --delay 0.5 -x run

# Build for production
build:
    @echo "Building for production..."
    cargo build --release

# Install dependencies
install:
    @echo "Installing dependencies..."
    npm install

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    rm -rf node_modules

# Format code
fmt:
    @echo "Formatting code..."
    cargo fmt

# Run tests
test:
    @echo "Running tests..."
    cargo test

# Show available recipes
help:
    @just --list
