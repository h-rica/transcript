# Transcript

Offline-first audio transcription desktop app built entirely in Rust.

Transcribe MP3, WAV, and M4A files locally — no internet required, no data leaves your device.

---

## Features

- **Offline-first** — all processing runs on your machine
- **Speaker diarization** — identifies who said what (via VibeVoice-ASR)
- **Real-time streaming** — segments appear as they are transcribed
- **Hardware-aware** — automatically recommends a model based on your RAM and CPU
- **Export** — TXT and SRT formats
- **Cross-platform** — Windows, macOS, Linux

## Stack

| Layer | Technology |
|---|---|
| Desktop framework | [Tauri v2](https://tauri.app) |
| Frontend | [Leptos 0.7](https://leptos.dev) — full Rust, WebAssembly |
| ASR runtime | [ort](https://ort.pyke.io) — ONNX tokenizers + candle Qwen2.5 decoder |
| Audio decode | [Symphonia](https://github.com/pdeljanov/Symphonia) — MP3 / WAV / M4A |
| UI components | [Singlestage UI](https://singlestage.doordesk.net) + Tailwind CSS |
| Utilities | [leptos-use](https://leptos-use.rs) |

## Models

| Model | Size | Diarization | Tier |
|---|---|---|---|
| Whisper Tiny | 150 MB | — | Bundled |
| Whisper Medium | 1.5 GB | — | 16 GB RAM |
| Whisper Large v3 | 3.1 GB | — | 16 GB RAM |
| VibeVoice INT8 | 8.5 GB | ✓ | 32 GB RAM |

ONNX artifacts: [MiicaLabs/vibevoice-onnx-artifacts](https://huggingface.co/MiicaLabs/vibevoice-onnx-artifacts)

## Getting started

### Prerequisites

```bash
# Rust stable
rustup update stable
rustup target add wasm32-unknown-unknown

# Tauri CLI
cargo install tauri-cli --version "^2"

# Trunk (WASM bundler)
cargo install trunk

# Node.js LTS (required by Tauri)
# https://nodejs.org
```

### Run in development

```bash
git clone https://github.com/miica-labs/transcript.git
cd transcript
cargo tauri dev
```

### Build for production

```bash
cargo tauri build
```

## Project structure

```
transcript/
├── src-tauri/          # Rust backend — ASR pipeline, IPC commands
│   └── src/
│       ├── commands/   # Tauri IPC command handlers
│       ├── asr/        # ONNX tokenizers + decoder pipeline
│       ├── audio/      # Symphonia audio decoder
│       ├── models/     # Model registry + downloader
│       └── export/     # TXT / SRT export
├── src/                # Leptos frontend — WebAssembly
│   ├── components/     # Reusable UI components
│   ├── pages/          # Screen-level components
│   └── state/          # Global signals and context
├── models/             # Model registry (TOML)
└── justfile            # Dev task runner
```

## Roadmap

| Phase | Status | Scope |
|---|---|---|
| Phase 0 — ONNX validation | ✅ Complete | VibeVoice-ASR tokenizers validated |
| Phase 1 — POC v0.1 | 🔄 In progress | Core transcription pipeline |
| Phase 2 — Beta v0.2 | 🔲 Planned | Model manager, resumable downloads |
| Phase 3 — v0.3 | 🔲 Planned | Live mic, LLM post-processing, DOCX export |
| Phase 4 — v1.0 | 🔲 Planned | Signed, audited, multilingual |

## Related

- [transcript-vibevoice-onnx](https://github.com/h-rica/transcript-vibevoice-onnx) — ONNX export demo repo
- [MiicaLabs/vibevoice-onnx-artifacts](https://huggingface.co/MiicaLabs/vibevoice-onnx-artifacts) — ONNX artifacts on HuggingFace

## License

Apache 2.0 — see [LICENSE](LICENSE)
