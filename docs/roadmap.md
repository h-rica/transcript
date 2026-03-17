# Transcript - Roadmap

**Version**: 0.2.0  
**Updated**: March 2026

---

## Phase 0 - Foundations Completed

**Deliverable**: POC validation of the ONNX hybrid strategy

| Task | Status |
| --- | --- |
| VibeVoice-ASR ONNX export (acoustic + semantic) | Complete |
| Numerical validation PyTorch vs ONNX | Complete, GO (5/5) |
| CPU benchmark - RTFx > 1.0 | Complete, 1.1-1.3x |
| Rust `ort` crate - compile + session load | Complete |
| CI GitHub Actions - validate + benchmark | Complete |
| ONNX artifacts on HuggingFace | Complete, `MiicaLabs/vibevoice-onnx-artifacts` |

---

## Phase 1 - POC (v0.1)

**Deliverable**: Working app on macOS, Windows, and Linux with Whisper Tiny available out of the box and VibeVoice INT8 downloadable in-app

| Week | Focus |
| --- | --- |
| 1-2 | Tauri v2 + Leptos project scaffold, dev environment |
| 3-4 | Audio pipeline: Symphonia decode -> PCM -> Whisper fallback path |
| 5-6 | ONNX tokenizers integrated with `ort`, Qwen2.5 decoder spike |
| 7 | Leptos UI: drop zone, preview, progress, transcript view |
| 8 | Export TXT/SRT, model download, packaging |

**Success criteria**

- Import MP3, WAV, or M4A and transcribe French or English
- Manual language selection only for v0.1 (`FR` or `EN`)
- Timestamps plus speaker identification when using VibeVoice INT8
- Export TXT and SRT
- Install base app without forcing a large model download up front

**Notes**

- `Auto` language detection is not part of v0.1. Full multilingual auto-detection remains planned for v0.3.
- The only concrete model variants in scope for v0.1 are `Whisper Tiny` and `VibeVoice INT8`.

---

## Phase 2 - Consolidation (v0.2)

**Deliverable**: Stable public beta

- Model manager UI (download, delete, storage)
- Hardware tier detection -> auto model recommendation
- Resumable downloads + SHA256 verification
- Error handling and recovery
- Settings screen
- CI/CD for app binaries (GitHub Releases)

---

## Phase 3 - Features (v0.3)

**Deliverable**: Feature-complete release candidate

- Live microphone transcription
- LLM post-processing (punctuation, summary via Candle)
- DOCX + JSON export
- Multilingual auto-detection
- Keyboard shortcuts
- Multiple simultaneous transcriptions

---

## Phase 4 - v1.0

**Deliverable**: Signed, audited public release

- Code audit + dependency review
- macOS notarization + Windows code signing
- Linux AppImage + Flatpak
- Documentation site
- Performance profiling + optimization
