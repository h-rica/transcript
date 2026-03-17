# Transcript — Tasks

**Version** : 0.2.0
**Updated** : 2026-03-17

---

## Phase 0 — ONNX Validation ✅ Complete

**Duration:** 2 weeks
**Repo:** [transcript-vibevoice-onnx](https://github.com/h-rica/transcript-vibevoice-onnx)

### Completed

- [x] Export Acoustic Tokenizer → ONNX (2.5 MB, opset 18)
- [x] Export Semantic Tokenizer → ONNX (2.5 MB, opset 128)
- [x] Numerical validation PyTorch vs ONNX — GO (5/5)
- [x] CPU benchmark — RTFx 1.1–1.3× on i7-10610U
- [x] Rust `ort` 2.0.0-rc.12 — cargo build on Windows / macOS / Linux
- [x] GitHub Actions CI — validate + benchmark + rust build (3 OS)
- [x] ONNX artifacts uploaded to HuggingFace (`MiicaLabs/vibevoice-onnx-artifacts`)
- [x] justfile with `export`, `validate`, `benchmark`, `upload-onnx`
- [x] pyproject.toml + uv setup

---

## Phase 1 — POC v0.1

**Duration:** 8 weeks
**Goal:** Working transcription on 3 OS, Whisper Tiny bundled

### Sprint 1 — Project scaffold

- [x] GitHub repo created (`h-rica/transcript`)
- [x] `cargo tauri init` — Tauri v2 + Trunk + Leptos
- [x] Frontend structure: `src/components`, `src/pages`, `src/state`
- [x] Backend structure: `src-tauri/src/commands`, `asr`, `audio`, `models`, `export`
- [x] `src-tauri/Cargo.toml` — ort, symphonia, sysinfo, reqwest, tokio
- [x] `Cargo.toml` — leptos 0.8, leptos-use, singlestage, tauri-sys
- [x] `index.html` + `Trunk.toml`
- [x] `src/main.rs` — Leptos 0.8 entry point
- [x] Add Tailwind CSS + Singlestage UI
- [x] `justfile` for the main project
- [x] `models/registry.toml` — model catalog (Whisper Tiny, VibeVoice INT8)
- [x] Backend stub modules (`mod.rs` for all submodules)
- [x] `cargo tauri dev` — app window opens

### Sprint 2 — Audio pipeline

- [x] `audio/decoder.rs` — Symphonia decode MP3/WAV/M4A → PCM f32 24kHz
- [x] `commands/audio.rs` — `get_audio_info` IPC command
- [x] Unit test: decode test files, assert sample rate + duration
- [x] whisper.cpp integration via FFI
    - [x] Add whisper-rs dependency
    - [x] Bundle Whisper Tiny model in installer
    - [x] `asr/whisper.rs` — run inference, return Vec<Segment>
- [x] `commands/transcribe.rs` — `transcribe_file` + `cancel_transcription`
- [x] Tauri event emission: `transcription_progress`, `transcription_segment`, `transcription_complete`
- [ ] Manual test: transcribe a 5-min MP3, segments appear in logs

### Sprint 3 — ONNX tokenizers integration

- [x] Bundle validated `.onnx` + `.onnx.data` artifacts and resolve them from app resources/app data
- [x] `asr/acoustic.rs` — load `vibevoice_acoustic.onnx`, run inference
- [x] `asr/semantic.rs` — load `vibevoice_semantic.onnx`, run inference
- [x] `asr/pipeline.rs` — orchestrate acoustic + semantic → segment stream (decoder output is still a placeholder segment)
- [x] Wire pipeline into `commands/transcribe.rs`
- [ ] Test: RTFx ≥ 1.0 on dev machine
- [x] `asr/decoder.rs` — stub for Qwen2.5 (Phase 2 placeholder)

### Sprint 4 — Leptos UI

- [ ] `src/app.rs` — Router + context providers (`hardware_info`, `settings`) (router + app state provider implemented; dedicated hardware/settings providers still pending)
- [x] `src/pages/home.rs` — DropZone + RecentList
- [ ] `src/pages/file_preview.rs` — FileInfoCard + ModelSelector + EstimateBox
- [ ] `src/pages/transcription.rs` — ProgressBar + LiveSegmentList + SpeedMeter
- [ ] `src/pages/transcript_view.rs` — Tabs (Speakers / Timeline / Raw) + ExportPanel
- [ ] `src/components/sidebar.rs` — Singlestage Sidebar (basic sidebar implemented; design-system version still pending)
- [ ] `src/components/drop_zone.rs` — leptos-use `use_drop_zone` (logic currently lives inline in `src/pages/home.rs`)
- [ ] `src/components/live_segment_list.rs` — reactive Vec<Segment> + auto-scroll
- [ ] `src/components/progress_bar.rs` — Singlestage Progress + Tauri event listener
- [ ] `src/state/app_state.rs` — global signals (hardware_info, settings, active_model) (`selected_file`, `selected_model`, `hardware_info`, and `active_model` exist; settings state is still missing)
- [ ] Dark mode via `leptos_darkmode`
- [ ] End-to-end test: drop file → transcription → view segments

### Sprint 5 - Export, model download, packaging

- [ ] `export/txt.rs` — TXT with optional timestamps + speaker labels
- [ ] `export/srt.rs` — SRT subtitles
- [ ] `commands/export.rs` — `export_transcript` + native file picker (stub exists)
- [ ] `models/downloader.rs` — reqwest stream download + SHA256 verification
- [ ] `models/hardware.rs` — sysinfo tier detection
- [ ] `models/registry.rs` — load `models/registry.toml`
- [ ] `commands/models.rs` — `get_models`, `download_model`, `delete_model` (stubs exist)
- [ ] `src/pages/model_manager.rs` — ModelCard + DownloadProgress + StorageBar (route exists; page is still placeholder content)
- [ ] `src/pages/settings.rs` — ToggleRow + SelectRow + SettingsStore (route exists; page is still placeholder content)
- [ ] `cargo tauri build` — installer on Windows
- [ ] Installer size check < 50 MB

### Immediate next tasks — 2026-03-17 audit

1. Run a manual 5-minute transcription and confirm segment/progress events in the UI.
2. Benchmark the ONNX path on the dev machine and record RTFx.
3. Run an installer/dev build verification to confirm bundled Whisper and ONNX assets resolve correctly outside the repo checkout.
4. Turn the current UI skeleton into a working flow: extract `drop_zone`, build `file_preview`, and wire `transcription` to Tauri events.
5. After the first end-to-end flow works, implement hardware detection, model APIs, and TXT/SRT export.

---

## Phase 2 — Consolidation v0.2

**Duration:** 6 weeks
**Goal:** Stable public beta

### Model management

- [ ] Resumable downloads (HTTP Range header)
- [ ] Download queue (multiple models in sequence)
- [ ] Add Whisper Base (300 MB) to registry
- [ ] Add VibeVoice INT4 (5.2 GB) to registry
- [ ] Model update check (compare local SHA256 vs HF)

### Export

- [ ] DOCX export via a Rust docx library
- [ ] Export queue (multiple formats at once)

### UI polish

- [ ] Onboarding screen (first launch)
- [ ] Error states for all screens (file format, RAM, network, SHA256 failure)
- [ ] Loading skeletons (Singlestage Skeleton)
- [ ] Toast notifications (download complete, export saved)
- [ ] Keyboard shortcuts (Cmd/Ctrl+O to open, Cmd/Ctrl+E to export)

### Settings

- [ ] Keep model in memory toggle
- [ ] DOCX export format option
- [ ] Reset all settings action

### CI/CD

- [ ] GitHub Actions — build installers on push to `main`
- [ ] Release workflow on `v*` tags
- [ ] Auto-sign macOS (notarization)
- [ ] Auto-sign Windows (code signing)
- [ ] Linux AppImage

---

## Phase 3 — Features v0.3

**Duration:** 8 weeks
**Goal:** Feature-complete release candidate

### Live microphone

- [ ] Microphone input via CPAL or Tauri plugin
- [ ] Real-time PCM streaming to ASR pipeline
- [ ] Live transcript view (no file import needed)
- [ ] Start/stop recording controls

### Qwen2.5 decoder integration

- [ ] `asr/decoder.rs` — full candle-transformers implementation
- [ ] Download Qwen2.5 SafeTensors from HuggingFace
- [ ] Wire decoder into ONNX pipeline
- [ ] Structured JSON output [{speaker, start, end, text, lang}]
- [ ] Remove whisper.cpp dependency for VibeVoice models

### LLM post-processing

- [ ] Punctuation restoration pass (candle)
- [ ] Summary generation
- [ ] Chapter detection
- [ ] Speaker name suggestion (rename Speaker A → detected name)

### Multilingual

- [ ] Automatic language detection
- [ ] Support 99+ languages via Whisper Large v3
- [ ] Language display in segment headers

### Export

- [ ] JSON export
- [ ] PDF export

### Additional models

- [ ] Add Voxtral Mini 3B (3.2 GB, 99 langs) to registry
- [ ] Add VibeVoice FP16 (14 GB, Pro tier)

---

## Phase 4 — v1.0

**Duration:** 4 weeks
**Goal:** Signed, audited public release

### Security and quality

- [ ] Dependency audit (`cargo audit`)
- [ ] Memory safety review (unsafe blocks audit)
- [ ] macOS notarization (Apple Developer ID)
- [ ] Windows code signing (EV certificate)
- [ ] Linux Flatpak submission

### Distribution

- [ ] GitHub Releases with signed installers
- [ ] Documentation website
- [ ] Homebrew cask (macOS)
- [ ] winget package (Windows)

### Performance

- [ ] Profile startup time — target < 2s
- [ ] Profile memory usage during transcription
- [ ] Optimize ONNX Level3 loading on capable hardware
- [ ] Benchmark on representative hardware tiers

---

## Backlog (unscheduled)

- [ ] Multi-window support (transcribe while reviewing another)
- [ ] Transcript search (full-text across all history)
- [ ] Speaker profile persistence (name your speakers across files)
- [ ] Plugin system for custom ASR models
- [ ] Collaborative sharing (optional, privacy-first)
- [ ] iOS / Android (Tauri Mobile, post v1.0)
