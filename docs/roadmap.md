# Transcript — Roadmap

**Version** : 0.2.0
**Updated** : March 2026

---

## Phase 0 — Foundations ✅ Complete

**Deliverable** : POC validation of the ONNX hybrid strategy

| Task | Status |
|---|---|
| VibeVoice-ASR ONNX export (acoustic + semantic) | ✅ |
| Numerical validation PyTorch vs ONNX | ✅ GO (5/5) |
| CPU benchmark — RTFx > 1.0 | ✅ 1.1–1.3× |
| Rust `ort` crate — compile + session load | ✅ |
| CI GitHub Actions — validate + benchmark | ✅ |
| ONNX artifacts on HuggingFace | ✅ MiicaLabs/vibevoice-onnx-artifacts |

---

## Phase 1 — POC (v0.1)

**Deliverable** : Working app on 3 OS, Whisper Tiny bundled

| Week | Focus |
|---|---|
| 1–2 | Tauri v2 + Leptos project scaffold, dev environment |
| 3–4 | Audio pipeline: Symphonia decode → PCM → whisper.cpp |
| 5–6 | ONNX tokenizers integrated (ort), Qwen2.5 decoder spike |
| 7 | Leptos UI: drop zone, progress, transcript view |
| 8 | Export TXT/SRT, model download, packaging |

**Success criteria:**
- Import MP3/WAV/M4A, transcribe FR or EN
- Timestamps + speaker identification (VibeVoice)
- Export TXT + SRT
- Installer < 50 MB on all 3 platforms

---

## Phase 2 — Consolidation (v0.2)

**Deliverable** : Stable public beta

- Model manager UI (download, delete, storage)
- Hardware tier detection → auto model recommendation
- Resumable downloads + SHA256 verification
- Error handling & recovery
- Settings screen
- CI/CD for app binaries (GitHub Releases)

---

## Phase 3 — Features (v0.3)

**Deliverable** : Feature-complete release candidate

- Live microphone transcription
- LLM post-processing (punctuation, summary via candle)
- DOCX + JSON export
- Multilingual auto-detection
- Keyboard shortcuts
- Multiple simultaneous transcriptions

---

## Phase 4 — v1.0

**Deliverable** : Signed, audited public release

- Code audit + dependency review
- macOS notarization + Windows code signing
- Linux AppImage + Flatpak
- Documentation site
- Performance profiling + optimization
