set shell := ["powershell", "-NoLogo", "-Command"]

# List available recipes
default:
    @just --list

# Run app in development mode
dev:
    cargo tauri dev

# Build production installer
build:
    cargo tauri build

# Check frontend (WASM)
check-ui:
    cargo check

# Check backend (Tauri)
check-backend:
    cd src-tauri; cargo check

# Check everything
check: check-ui check-backend

# Run frontend tests
test-ui:
    cargo test

# Run backend tests
test-backend:
    cd src-tauri; cargo test

# Run all tests
test: test-ui test-backend

# Format all Rust code
fmt:
    cargo fmt
    cd src-tauri; cargo fmt

# Lint all Rust code
lint:
    cargo clippy -- -D warnings
    cd src-tauri; cargo clippy -- -D warnings

# Download Whisper Tiny bundled model (run once before cargo tauri build)
download-whisper-tiny:
    New-Item -ItemType Directory -Path src-tauri/resources -Force
    Invoke-WebRequest -Uri "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin" -OutFile "src-tauri/resources/ggml-tiny.bin"
    Write-Host "done: src-tauri/resources/ggml-tiny.bin"

# Clean build artifacts
clean:
    cargo clean
    cd src-tauri; cargo clean
    Remove-Item -ErrorAction SilentlyContinue output.css
    Remove-Item -ErrorAction SilentlyContinue dist -Recurse

# Clean and rebuild
rebuild: clean dev
