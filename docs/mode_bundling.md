# Transcript — Model bundling

How models are distributed, downloaded, and bundled in the Transcript installer.

---

## Bundled model — Whisper Tiny

Whisper Tiny (150 MB) is the only model bundled directly in the installer.
It guarantees the app works out of the box on any hardware, with no internet required.

### Download (maintainer only — run once)

```powershell
# Create the resources directory
New-Item -ItemType Directory -Path src-tauri/resources -Force

# Download Whisper Tiny from HuggingFace
Invoke-WebRequest `
  -Uri "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin" `
  -OutFile "src-tauri/resources/ggml-tiny.bin"
```

The file is gitignored — it must be downloaded locally before running `cargo tauri build`.

### Tauri bundle configuration

In `src-tauri/tauri.conf.json`, the model is mapped into the installer:

```json
"bundle": {
"resources": {
"resources/ggml-tiny.bin": "models/ggml-tiny.bin"
}
}
```

At runtime, Tauri exposes the bundled resource path via `app.path().resource_dir()`.

### Runtime path resolution

`commands/transcribe.rs` uses a two-step lookup:

1. **Bundled path** — checked first for `whisper-tiny` via `resource_dir()/models/ggml-tiny.bin`
2. **Downloaded path** — fallback to `data_dir()/transcript/models/<filename>` for all models

```rust
fn resolve_model_path(app: &AppHandle, model_id: &str) -> anyhow::Result<String> {
    if model_id == "whisper-tiny" {
        let bundled = app.path().resource_dir()?.join("models/ggml-tiny.bin");
        if bundled.exists() {
            return Ok(bundled.to_string_lossy().to_string());
        }
    }
    // fallback to downloaded models in data_dir
    ...
}
```

---

## Downloaded models

All other models are downloaded on demand from HuggingFace via the in-app Model Manager.

| Model            | Source repo                          | File                                                                                |
|------------------|--------------------------------------|-------------------------------------------------------------------------------------|
| Whisper Base     | `ggerganov/whisper.cpp`              | `ggml-base.bin`                                                                     |
| Whisper Medium   | `ggerganov/whisper.cpp`              | `ggml-medium.bin`                                                                   |
| Whisper Large v3 | `ggerganov/whisper.cpp`              | `ggml-large-v3.bin`                                                                 |
| VibeVoice INT8   | `MiicaLabs/vibevoice-onnx-artifacts` | `onnx/vibevoice_acoustic.onnx` + `.data` + `onnx/vibevoice_semantic.onnx` + `.data` |

Downloaded models are stored in the OS app data directory:

| Platform | Path                                                 |
|----------|------------------------------------------------------|
| Windows  | `C:\Users\<user>\AppData\Roaming\transcript\models\` |
| macOS    | `~/Library/Application Support/transcript/models/`   |
| Linux    | `~/.local/share/transcript/models/`                  |

### SHA256 verification

After download, each file is verified against the SHA256 hash defined in `models/registry.toml`.
A corrupted or incomplete download triggers an error and re-download prompt.

---

## .gitignore rules

Model files are excluded from git — they are large binary files that belong in object storage, not version control.

```gitignore
# src-tauri/.gitignore
resources/ggml-tiny.bin
resources/*.bin
resources/*.onnx
resources/*.data
```

---

## Adding a new model

1. Add entry to `models/registry.toml`
2. Add SHA256 hash (run `sha256sum <file>` or `Get-FileHash` on Windows)
3. Update `resolve_model_path()` in `commands/transcribe.rs` if needed
4. Test download flow in Model Manager UI