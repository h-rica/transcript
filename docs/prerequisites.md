# Transcript — Prerequisites

All tools required to build and develop Transcript.

---

## All platforms

```bash
# Rust stable
rustup update stable
rustup target add wasm32-unknown-unknown

# Tauri CLI
cargo install tauri-cli --version "^2"

# Trunk (WASM bundler for Leptos)
cargo install trunk

# Node.js LTS — required by Tauri build pipeline
# https://nodejs.org

# just — task runner
cargo install just
```

---

## Windows

### LLVM — required by whisper-rs (bindgen)

`whisper-rs` uses `bindgen` to generate Rust bindings for whisper.cpp.
`bindgen` requires `libclang` which is part of LLVM.

```powershell
# Install LLVM
winget install LLVM.LLVM

# Set LIBCLANG_PATH for the current session
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"

# Persist across sessions (run once)
[System.Environment]::SetEnvironmentVariable(
    "LIBCLANG_PATH",
    "C:\Program Files\LLVM\bin",
    "User"
)
```

Verify:

```powershell
clang --version
# clang version 19.x.x
```

### Visual Studio Build Tools

Required by Rust on Windows. Install via:

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```

Select: **Desktop development with C++**

### WebView2

Required by Tauri — usually pre-installed on Windows 11.
If missing: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

---

## macOS

```bash
# Xcode command line tools
xcode-select --install

# Homebrew
brew install llvm

# Add to shell profile (~/.zshrc or ~/.bash_profile)
export LIBCLANG_PATH=$(brew --prefix llvm)/lib
```

---

## Linux (Ubuntu / Debian)

```bash
sudo apt update
sudo apt install -y \
    libclang-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

---

## Environment variable summary

| Variable           | Value                      | Required for             |
|--------------------|----------------------------|--------------------------|
| `LIBCLANG_PATH`    | Path to LLVM `bin/`        | whisper-rs (bindgen)     |
| `ORT_LIB_LOCATION` | Path to OnnxRuntime `lib/` | ort crate (load-dynamic) |
| `RUST_LOG`         | e.g. `transcript=debug`    | Logging level            |