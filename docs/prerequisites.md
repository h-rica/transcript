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

### Visual Studio Build Tools

Required by Rust on Windows.

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```

Select: **Desktop development with C++**

### CMake — required by whisper-rs build system

```powershell
winget install Kitware.CMake
```

CMake installer adds itself to the System PATH automatically.
If `cmake --version` still fails after install, add manually:

1. `Win + R` → `sysdm.cpl`
2. **Advanced** → **Environment Variables**
3. **System variables** → `Path` → **Edit** → **New**
4. Add `C:\Program Files\CMake\bin`
5. **OK** everywhere, then reopen terminal

Verify (new terminal):

```powershell
cmake --version
```

### LLVM — required by whisper-rs (bindgen)

`whisper-rs` uses `bindgen` to generate Rust bindings for whisper.cpp.
`bindgen` requires `libclang` which ships with LLVM.

```powershell
# Install
winget install LLVM.LLVM

# Add to PATH for current session
$env:PATH += ";C:\Program Files\LLVM\bin"

# Persist PATH across sessions (run once)
[System.Environment]::SetEnvironmentVariable(
    "PATH",
    [System.Environment]::GetEnvironmentVariable("PATH", "User") + ";C:\Program Files\LLVM\bin",
    "User"
)

# Set LIBCLANG_PATH for current session
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"

# Persist LIBCLANG_PATH across sessions (run once)
[System.Environment]::SetEnvironmentVariable(
    "LIBCLANG_PATH",
    "C:\Program Files\LLVM\bin",
    "User"
)
```

### Windows SDK headers — fix stdbool.h not found

`bindgen` needs access to MSVC C runtime headers. Find your SDK version:

```powershell
ls "C:\Program Files (x86)\Windows Kits\10\Include"
# e.g. 10.0.26100.0
```

Set the include path (replace version number as needed):

```powershell
# Current session
$env:BINDGEN_EXTRA_CLANG_ARGS = "-IC:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0\ucrt"

# Persist (run once)
[System.Environment]::SetEnvironmentVariable(
    "BINDGEN_EXTRA_CLANG_ARGS",
    "-IC:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0\ucrt",
    "User"
)
```

### WebView2

Required by Tauri — pre-installed on Windows 11.
If missing: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

### Verify all tools (new terminal after installs)

```powershell
clang --version        # clang version 19.x.x
cmake --version        # cmake version 3.x.x
echo $env:LIBCLANG_PATH        # C:\Program Files\LLVM\bin
echo $env:BINDGEN_EXTRA_CLANG_ARGS   # -IC:\Program Files (x86)\...
```

---

## macOS

```bash
# Xcode command line tools
xcode-select --install

# Homebrew
brew install llvm cmake

# Add to shell profile (~/.zshrc or ~/.bash_profile)
export LIBCLANG_PATH=$(brew --prefix llvm)/lib
export PATH="$(brew --prefix llvm)/bin:$PATH"
```

---

## Linux (Ubuntu / Debian)

```bash
sudo apt update
sudo apt install -y \
    clang \
    cmake \
    libclang-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

---

## Environment variables summary

| Variable                   | Example value                                                        | Required for             |
|----------------------------|----------------------------------------------------------------------|--------------------------|
| `PATH`                     | `...;C:\Program Files\LLVM\bin`                                      | clang / bindgen          |
| `LIBCLANG_PATH`            | `C:\Program Files\LLVM\bin`                                          | whisper-rs bindgen       |
| `BINDGEN_EXTRA_CLANG_ARGS` | `-IC:\Program Files (x86)\Windows Kits\10\Include\10.0.26100.0\ucrt` | stdbool.h fix (Windows)  |
| `ORT_LIB_LOCATION`         | path to OnnxRuntime `lib/`                                           | ort crate (load-dynamic) |
| `RUST_LOG`                 | `transcript=debug`                                                   | logging level            |