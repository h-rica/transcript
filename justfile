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

# Download VibeVoice ONNX artifacts from HuggingFace (run once)
download-onnx:
    New-Item -ItemType Directory -Path src-tauri/resources/onnx -Force
    Invoke-WebRequest -Uri "https://huggingface.co/MiicaLabs/vibevoice-onnx-artifacts/resolve/main/onnx/vibevoice_acoustic.onnx" -OutFile "src-tauri/resources/onnx/vibevoice_acoustic.onnx"
    Invoke-WebRequest -Uri "https://huggingface.co/MiicaLabs/vibevoice-onnx-artifacts/resolve/main/onnx/vibevoice_acoustic.onnx.data" -OutFile "src-tauri/resources/onnx/vibevoice_acoustic.onnx.data"
    Invoke-WebRequest -Uri "https://huggingface.co/MiicaLabs/vibevoice-onnx-artifacts/resolve/main/onnx/vibevoice_semantic.onnx" -OutFile "src-tauri/resources/onnx/vibevoice_semantic.onnx"
    Invoke-WebRequest -Uri "https://huggingface.co/MiicaLabs/vibevoice-onnx-artifacts/resolve/main/onnx/vibevoice_semantic.onnx.data" -OutFile "src-tauri/resources/onnx/vibevoice_semantic.onnx.data"
    Write-Host "done: src-tauri/resources/onnx/"

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
