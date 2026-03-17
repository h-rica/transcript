# Transcript — Technical Architecture

**Version** : 0.2.0
**Updated** : March 2026
**Stack** : Tauri v2 + Leptos · Full Rust

---

## 1. High-level architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        TRANSCRIPT APP                           │
│                                                                 │
│  ┌─────────────────────┐      ┌──────────────────────────────┐ │
│  │   FRONTEND (Leptos) │      │      BACKEND (Rust)          │ │
│  │                     │ IPC  │                              │ │
│  │  • File drop zone   │◄────►│  • ASR pipeline              │ │
│  │  • Progress stream  │      │  • Model manager             │ │
│  │  • Transcript view  │      │  • Hardware detector         │ │
│  │  • Speaker timeline │      │  • Audio decoder             │ │
│  │  • Export panel     │      │  • Export engine             │ │
│  └─────────────────────┘      └──────────┬───────────────────┘ │
│                                          │                      │
│              ┌───────────────────────────┼──────────────┐      │
│              │                           │              │      │
│   ┌──────────▼──────────┐  ┌─────────────▼────┐  ┌─────▼────┐ │
│   │  ASR Runtime        │  │    Symphonia     │  │   FS     │ │
│   │                     │  │  (audio decode)  │  │          │ │
│   │  Acoustic ONNX      │  └──────────────────┘  └──────────┘ │
│   │  Semantic ONNX      │                                      │
│   │  Qwen2.5 (candle)   │                                      │
│   │  ── or ──           │                                      │
│   │  whisper.cpp (FFI)  │                                      │
│   └─────────────────────┘                                      │
│                                                                 │
│  Offline-first — no network calls during transcription          │
│  Network only for: model download, optional update check        │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Tech stack

### Core

| Layer | Technology | Version | Rationale |
|---|---|---|---|
| Desktop framework | Tauri | v2.x | ~5MB binary, native system access, cross-platform |
| Frontend | Leptos | 0.7.x | Full Rust, fine-grained reactivity, consistent stack |
| Language | Rust | 1.75+ stable | Performance, memory safety, rich crate ecosystem |

### ASR Runtime — ONNX Hybrid Strategy (POC Primary)

Validated during the `transcript-vibevoice-onnx` demo repo session.

| Component | Runtime | Method | Status |
|---|---|---|---|
| Acoustic Tokenizer | `ort` crate (Rust) | ONNX — exported via Python once | ✅ Validated |
| Semantic Tokenizer | `ort` crate (Rust) | ONNX — exported via Python once | ✅ Validated |
| Qwen2.5 Decoder | `candle-transformers` | SafeTensors direct | 🔲 To implement |

Fallback: `whisper.cpp` via FFI (MIT, CPU+GPU, Whisper Tiny bundled at install).

### Backend crates

| Module | Crate | Role |
|---|---|---|
| ONNX inference | `ort 2.0.0-rc.12` + `load-dynamic` | Acoustic + Semantic tokenizer inference |
| LLM inference | `candle-transformers` | Qwen2.5 decoder |
| Tokenizer | `tokenizers 0.19` | Text tokenization for Qwen2.5 |
| Audio decode | `symphonia` | MP3 / WAV / M4A → PCM 24kHz |
| Hardware detection | `sysinfo` | RAM, CPU tier detection |
| HTTP download | `reqwest` async | Model download with progress |
| Hash verification | `sha2` | SHA256 model integrity check |
| Serialization | `serde` + `toml` | Model registry, app config |
| Async runtime | `tokio` | Async ops, channel streaming |

### Frontend crates

| Module | Crate | Role |
|---|---|---|
| Reactivity | `leptos 0.7` | Reactive UI components |
| Tauri integration | `tauri-sys` | IPC, file system, dialogs |
| Styling | Tailwind CSS | Via Tauri asset pipeline |

---

## 3. Project structure

```
transcript/
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── main.rs             # Tauri entry point
│   │   ├── commands/           # IPC command handlers
│   │   │   ├── transcribe.rs   # Transcription pipeline
│   │   │   ├── models.rs       # Model management
│   │   │   └── export.rs       # TXT / SRT export
│   │   ├── asr/                # ASR runtime
│   │   │   ├── pipeline.rs     # Main pipeline orchestrator
│   │   │   ├── acoustic.rs     # Acoustic tokenizer (ort)
│   │   │   ├── semantic.rs     # Semantic tokenizer (ort)
│   │   │   ├── decoder.rs      # Qwen2.5 decoder (candle)
│   │   │   └── whisper.rs      # whisper.cpp fallback
│   │   ├── audio/
│   │   │   └── decoder.rs      # Symphonia audio decoder
│   │   ├── models/
│   │   │   ├── registry.rs     # Model catalog
│   │   │   ├── downloader.rs   # HuggingFace download
│   │   │   └── hardware.rs     # Hardware tier detection
│   │   └── export/
│   │       ├── txt.rs
│   │       └── srt.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src/                        # Leptos frontend
│   ├── main.rs                 # Leptos entry point
│   ├── app.rs                  # Root component + routing
│   ├── components/
│   │   ├── drop_zone.rs        # File drop area
│   │   ├── progress.rs         # Live transcription progress
│   │   ├── transcript.rs       # Segment + speaker view
│   │   ├── model_selector.rs   # Hardware-aware model picker
│   │   └── export_panel.rs     # Export actions
│   └── state/
│       └── app_state.rs        # Global reactive state
│
├── models/                     # Model registry TOML
│   └── registry.toml
├── justfile                    # Dev task runner
├── pyproject.toml              # Python export tools (uv)
└── README.md
```

---

## 4. ASR pipeline

```
Audio file (MP3/WAV/M4A)
        │
        ▼
┌───────────────────┐
│  Symphonia        │  PCM 24kHz mono f32
│  Audio Decoder    │
└────────┬──────────┘
         │
         ├──────────────────────────────────────┐
         ▼                                      ▼
┌────────────────────┐              ┌──────────────────────┐
│ Acoustic Tokenizer │              │ Semantic Tokenizer   │
│ (ort ONNX)         │              │ (ort ONNX)           │
│ → [batch,frames,64]│              │ → [batch,frames,128] │
└────────┬───────────┘              └──────────┬───────────┘
         │                                     │
         └──────────────┬──────────────────────┘
                        ▼
              ┌─────────────────────┐
              │  Qwen2.5 Decoder    │
              │  (candle-transformers)│
              │                     │
              │  Structured JSON:   │
              │  [{speaker, start,  │
              │    end, text, lang}]│
              └─────────┬───────────┘
                        ▼
              ┌─────────────────────┐
              │  Segment Streaming  │
              │  (tokio channel)    │
              │  → IPC → Leptos UI  │
              └─────────────────────┘
```

Frame rate output: **~7.5 Hz** for both tokenizers (validated).
Benchmark on i7-10610U: **RTFx > 1.0** — faster than real-time on CPU.

---

## 5. Hardware tiers

Detected at startup via `sysinfo`, used to suggest the appropriate model:

| Tier | RAM | VRAM | Suggested Model |
|---|---|---|---|
| Minimal | 8 GB | None | Whisper Tiny (bundled, 150 MB) |
| Standard | 16 GB | 4–6 GB | Whisper Medium / VibeVoice INT4 |
| Comfortable | 32 GB | 8–12 GB | Whisper Large v3 / VibeVoice INT8 |
| Pro | 64 GB+ | 16 GB+ | VibeVoice FP16 |

---

## 6. Model registry

```toml
# models/registry.toml

[[models]]
id          = "whisper-tiny"
name        = "Whisper Tiny"
size_mb     = 150
tier        = "minimal"
bundled     = true
diarization = false
languages   = ["fr", "en", "multilingual"]
source      = "bundled"

[[models]]
id          = "vibevoice-int8"
name        = "VibeVoice-ASR INT8"
size_mb     = 8500
tier        = "standard"
bundled     = false
diarization = true
languages   = ["fr", "en"]
source      = "huggingface"
repo_id     = "MiicaLabs/vibevoice-onnx-artifacts"
files       = [
    "onnx/vibevoice_acoustic.onnx",
    "onnx/vibevoice_acoustic.onnx.data",
    "onnx/vibevoice_semantic.onnx",
    "onnx/vibevoice_semantic.onnx.data",
]
sha256      = { "vibevoice_acoustic.onnx" = "...", "vibevoice_semantic.onnx" = "..." }
```

---

## 7. IPC — Tauri commands

| Command | Direction | Payload |
|---|---|---|
| `transcribe_file` | Frontend → Backend | `{ path, model_id, language }` |
| `transcription_progress` | Backend → Frontend (event) | `{ percent, current_segment }` |
| `transcription_segment` | Backend → Frontend (event) | `TranscriptSegment` |
| `transcription_complete` | Backend → Frontend (event) | `TranscriptResult` |
| `get_models` | Frontend → Backend | — |
| `download_model` | Frontend → Backend | `{ model_id }` |
| `download_progress` | Backend → Frontend (event) | `{ model_id, percent, speed_mbps }` |
| `export_transcript` | Frontend → Backend | `{ format, path, result }` |
| `get_hardware_info` | Frontend → Backend | — |

---

## 8. POC scope (v0.1)

**In scope:**
- Audio import MP3 / WAV / M4A
- Offline transcription FR + EN (manual language selection)
- Timestamps per segment
- Speaker identification (VibeVoice native diarization)
- Real-time segment streaming to UI
- Export TXT + SRT
- Hardware tier detection → model recommendation
- In-app model download with progress + SHA256 verification
- Whisper Tiny bundled in installer

**Out of scope for v0.1:**
- Live microphone streaming
- DOCX / PDF / JSON export
- Multilingual auto-detection
- LLM post-processing (punctuation, summary)
- Cloud sync
- Auto-update
- CUDA on Windows

---

## 9. Key architectural decisions

**ADR-01 — Full Rust stack (Tauri + Leptos)**
Rationale: single language, ~5 MB binary, no Node.js runtime, consistent toolchain.
Trade-off: steeper learning curve than React/Next.js.

**ADR-02 — ONNX Hybrid for VibeVoice-ASR**
Rationale: Acoustic + Semantic tokenizers → ONNX (validated, RTFx > 1.0 on CPU). Qwen2.5 decoder → candle-transformers (no Python at runtime).
Trade-off: export step requires Python + 18 GB disk space (one-time, maintainer only).

**ADR-03 — whisper.cpp as bundled fallback**
Rationale: Whisper Tiny (150 MB) works on any hardware, covers the Minimal tier, ensures the app always works out of the box.

**ADR-04 — Symphonia for audio decoding**
Rationale: pure Rust, no ffmpeg dependency, supports all target formats.

**ADR-05 — `ort` crate with load-dynamic**
Rationale: external OnnxRuntime `.dll/.so`, smaller binary, updatable without recompile. Use `GraphOptimizationLevel::Level1` during development — Level3 causes 2h+ load on laptop CPUs (validated on i7-10610U, 842 nodes).