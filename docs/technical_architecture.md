# Transcript - Technical Architecture

**Version**: 0.2.0  
**Updated**: March 2026  
**Stack**: Tauri v2 + Leptos + Rust

---

## 1. High-level architecture

```
+---------------------------------------------------------------+
|                        TRANSCRIPT APP                         |
|                                                               |
|  +----------------------+     +----------------------------+  |
|  | Frontend (Leptos)    | IPC | Backend (Rust)            |  |
|  |                      |<--->|                            |  |
|  | - File drop zone     |     | - ASR pipeline            |  |
|  | - Preview + settings |     | - Model manager           |  |
|  | - Progress stream    |     | - Hardware detector       |  |
|  | - Transcript view    |     | - Audio decoder           |  |
|  | - Export panel       |     | - Export engine           |  |
|  +----------------------+     +-------------+--------------+  |
|                                                |               |
|                     +--------------------------+------------+  |
|                     |                          |            |  |
|             +-------v--------+        +--------v------+ +---v--+
|             | ASR runtime    |        | Symphonia     | | FS   |
|             |                |        | audio decode  | |      |
|             | - Acoustic ONNX|        +---------------+ +------+
|             | - Semantic ONNX|
|             | - Qwen2.5      |
|             | - or whisper   |
|             +----------------+
|                                                               |
|  Offline-first: no network calls during transcription         |
|  Network only for model download and optional update checks   |
+---------------------------------------------------------------+
```

---

## 2. Tech stack

### Core

| Layer | Technology | Version | Rationale |
| --- | --- | --- | --- |
| Desktop framework | Tauri | v2.x | Small native shell, system access, cross-platform |
| Frontend | Leptos | 0.7.x | Full Rust UI stack with fine-grained reactivity |
| Language | Rust | 1.75+ stable | Performance, safety, and crate ecosystem |

### ASR runtime - ONNX hybrid strategy

Validated during the `transcript-vibevoice-onnx` demo repo work.

| Component | Runtime | Method | Status |
| --- | --- | --- | --- |
| Acoustic tokenizer | `ort` crate | ONNX exported offline | Validated |
| Semantic tokenizer | `ort` crate | ONNX exported offline | Validated |
| Qwen2.5 decoder | `candle-transformers` | SafeTensors direct | Planned |

Fallback: `whisper.cpp` via FFI, with Whisper Tiny available for minimal hardware.

### Backend crates

| Module | Crate | Role |
| --- | --- | --- |
| ONNX inference | `ort 2.0.0-rc.12` + `load-dynamic` | Acoustic + semantic tokenizer inference |
| LLM inference | `candle-transformers` | Qwen2.5 decoder |
| Tokenizer | `tokenizers 0.19` | Text tokenization for Qwen2.5 |
| Audio decode | `symphonia` | MP3 / WAV / M4A -> PCM 24kHz |
| Hardware detection | `sysinfo` | RAM and CPU tier detection |
| HTTP download | `reqwest` | Model download with progress |
| Hash verification | `sha2` | SHA256 model integrity check |
| Serialization | `serde` + `toml` | Model registry and app config |
| Async runtime | `tokio` | Async work and event streaming |

### Frontend crates

| Module | Crate | Role |
| --- | --- | --- |
| Reactivity | `leptos 0.7` | Reactive UI components |
| Tauri integration | `tauri-sys` | IPC, dialogs, system APIs |
| Styling | Tailwind CSS | App styling pipeline |

---

## 3. Project structure

This reflects the current repository layout more closely than the original draft.

```
transcript/
|-- src-tauri/
|   |-- src/
|   |   |-- main.rs
|   |   |-- lib.rs
|   |   |-- commands/
|   |   |   |-- audio.rs
|   |   |   |-- export.rs
|   |   |   |-- hardware.rs
|   |   |   |-- models.rs
|   |   |   |-- settings.rs
|   |   |   `-- transcribe.rs
|   |   |-- asr/
|   |   |   |-- acoustic.rs
|   |   |   |-- pipeline.rs
|   |   |   |-- semantic.rs
|   |   |   `-- whisper.rs
|   |   |-- audio/
|   |   |   `-- decoder.rs
|   |   |-- models/
|   |   |   |-- registry.rs
|   |   |   |-- downloader.rs
|   |   |   `-- hardware.rs
|   |   `-- export/
|   |       |-- txt.rs
|   |       `-- srt.rs
|   `-- tauri.conf.json
|
|-- src/
|   |-- main.rs
|   |-- app.rs
|   |-- components/
|   |   |-- drop_zone.rs
|   |   `-- sidebar.rs
|   |-- pages/
|   |   |-- home.rs
|   |   |-- file_preview.rs
|   |   |-- transcription.rs
|   |   |-- transcript_view.rs
|   |   |-- model_manager.rs
|   |   `-- settings.rs
|   `-- state/
|       `-- app_state.rs
|
|-- models/
|   `-- registry.toml
|-- justfile
`-- README.md
```

---

## 4. ASR pipeline

```
Audio file (MP3/WAV/M4A)
        |
        v
+-------------------+
| Symphonia decoder |
| PCM 24kHz mono    |
+---------+---------+
          |
          +-----------------------------+
          v                             v
+---------------------+       +----------------------+
| Acoustic tokenizer  |       | Semantic tokenizer   |
| ort ONNX            |       | ort ONNX             |
| -> [batch,frames,64]|       | -> [batch,frames,128]|
+----------+----------+       +----------+-----------+
           |                             |
           +-------------+---------------+
                         v
               +----------------------+
               | Qwen2.5 decoder      |
               | candle-transformers  |
               | -> structured result |
               +----------+-----------+
                          v
               +----------------------+
               | Segment streaming    |
               | tokio -> IPC -> UI   |
               +----------------------+
```

Validated output rate: about 7.5 Hz for both tokenizers.  
Benchmark on i7-10610U: RTFx > 1.0, faster than real time on CPU.

---

## 5. Hardware tiers

Detected at startup via `sysinfo`, used to suggest the appropriate currently supported model:

| Tier | RAM | VRAM | Suggested model |
| --- | --- | --- | --- |
| Minimal | 8 GB | None | Whisper Tiny |
| Standard | 16 GB | 4-6 GB | VibeVoice INT8 if downloaded, otherwise Whisper Tiny |
| Comfortable | 32 GB | 8-12 GB | VibeVoice INT8 |
| Pro | 64 GB+ | 16 GB+ | VibeVoice INT8 |

**Note**: `VibeVoice INT4` and `VibeVoice FP16` are not defined as concrete artifacts in the current registry. If those variants are created later, they should be added as explicit registry entries and roadmap items before becoming recommendation targets.

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
languages   = ["fr", "en"]
source      = "bundled"

[[models]]
id          = "vibevoice-int8"
name        = "VibeVoice INT8"
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
sha256 = {
    "vibevoice_acoustic.onnx" = "...",
    "vibevoice_semantic.onnx" = "..."
}
```

---

## 7. IPC - Tauri commands

| Command | Direction | Payload |
| --- | --- | --- |
| `transcribe_file` | Frontend -> Backend | `{ path, model_id, language }` |
| `transcription_progress` | Backend -> Frontend event | `{ percent, elapsed }` |
| `transcription_segment` | Backend -> Frontend event | `TranscriptSegment` |
| `transcription_complete` | Backend -> Frontend event | `TranscriptResult` |
| `get_models` | Frontend -> Backend | none |
| `download_model` | Frontend -> Backend | `{ model_id }` |
| `download_progress` | Backend -> Frontend event | `{ model_id, percent, speed_mbps }` |
| `export_transcript` | Frontend -> Backend | `{ format, path, result }` |
| `get_hardware_info` | Frontend -> Backend | none |
| `get_audio_info` | Frontend -> Backend | `{ path }` |
| `get_settings` | Frontend -> Backend | none |
| `save_settings` | Frontend -> Backend | `SettingsStore` |

For v0.1, `language` is expected to be an explicit manual choice such as `fr` or `en`. `auto` is not part of the v0.1 contract.

---

## 8. POC scope (v0.1)

**In scope**

- Audio import: MP3, WAV, M4A
- Offline transcription for French and English
- Manual language selection only
- Timestamps per segment
- Speaker identification with VibeVoice INT8
- Real-time segment streaming to the UI
- Export TXT + SRT
- Hardware tier detection -> model recommendation
- In-app model download with progress + SHA256 verification
- Whisper Tiny available from install time

**Out of scope for v0.1**

- Live microphone streaming
- DOCX / PDF / JSON export
- Multilingual auto-detection
- LLM post-processing (punctuation, summary)
- Cloud sync
- Auto-update
- Resumable downloads
- CUDA on Windows

---

## 9. Key architectural decisions

**ADR-01 - Full Rust stack (Tauri + Leptos)**  
Rationale: one language, small desktop footprint, no Node.js runtime, consistent toolchain.  
Trade-off: steeper learning curve than a JS-first frontend stack.

**ADR-02 - ONNX hybrid for VibeVoice-ASR**  
Rationale: acoustic + semantic tokenizers in ONNX are already validated on CPU; Qwen2.5 decoder stays in Rust via Candle.  
Trade-off: export tooling still requires Python and significant maintainer-side disk space.

**ADR-03 - Whisper Tiny as fallback path**  
Rationale: works on minimal hardware and guarantees the app can transcribe immediately, even before larger model downloads finish.

**ADR-04 - Symphonia for audio decoding**  
Rationale: pure Rust, no FFmpeg dependency, covers the target input formats.

**ADR-05 - `ort` crate with `load-dynamic`**  
Rationale: external ONNX Runtime shared libraries keep the binary smaller and allow runtime replacement without recompiling.  
Operational note: `GraphOptimizationLevel::Level1` is preferred in development because `Level3` caused extremely long load times on laptop CPUs during validation.
