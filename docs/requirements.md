# Transcript — Requirements

**Version** : 0.2.0
**Updated** : March 2026
**Status** : Phase 1 in progress

---

## 1. Project overview

Transcript is an offline-first desktop application for audio transcription. All processing runs locally — no audio or transcript data leaves the user's machine.

**Core principles:**
- Offline-first — no internet required during transcription
- Privacy by design — no telemetry by default, no cloud sync
- Hardware-aware — model selection adapts to the machine's capabilities
- Open source — Apache 2.0

---

## 2. Functional requirements

### 2.1 Audio import

| ID | Requirement | Phase |
|---|---|---|
| F-01 | Import MP3 files | v0.1 |
| F-02 | Import WAV files | v0.1 |
| F-03 | Import M4A files | v0.1 |
| F-04 | Import via drag-and-drop | v0.1 |
| F-05 | Import via file browser | v0.1 |
| F-06 | Support files up to 2 hours | v0.1 |
| F-07 | Import FLAC files | v0.2 |
| F-08 | Import OGG files | v0.2 |
| F-09 | Live microphone input | v0.3 |

### 2.2 Transcription

| ID | Requirement | Phase |
|---|---|---|
| F-10 | Offline transcription — no network during processing | v0.1 |
| F-11 | Manual language selection (FR / EN) | v0.1 |
| F-12 | Timestamps per segment | v0.1 |
| F-13 | Speaker identification (diarization) via VibeVoice-ASR | v0.1 |
| F-14 | Real-time segment streaming to UI | v0.1 |
| F-15 | Automatic language detection | v0.3 |
| F-16 | Multilingual support (99+ languages via Whisper Large) | v0.3 |
| F-17 | LLM post-processing (punctuation, summary) | v0.3 |

### 2.3 Export

| ID | Requirement | Phase |
|---|---|---|
| F-18 | Export as plain text (.txt) | v0.1 |
| F-19 | Export as SRT subtitles (.srt) | v0.1 |
| F-20 | Optional timestamps in TXT export | v0.1 |
| F-21 | Optional speaker labels in TXT export | v0.1 |
| F-22 | Export as Word document (.docx) | v0.2 |
| F-23 | Export as JSON | v0.3 |
| F-24 | Export as PDF | v0.3 |
| F-25 | Copy to clipboard | v0.1 |

### 2.4 Model management

| ID | Requirement | Phase |
|---|---|---|
| F-26 | Whisper Tiny bundled in installer | v0.1 |
| F-27 | In-app model download with progress | v0.1 |
| F-28 | SHA256 verification after download | v0.1 |
| F-29 | Hardware tier detection (RAM, CPU, GPU) | v0.1 |
| F-30 | Hardware-aware model recommendation | v0.1 |
| F-31 | Model deletion with storage reclaim | v0.1 |
| F-32 | Resumable downloads | v0.2 |
| F-33 | Download queue (multiple models) | v0.2 |

### 2.5 Settings

| ID | Requirement | Phase |
|---|---|---|
| F-34 | Default language setting | v0.1 |
| F-35 | Default model setting (Auto / manual) | v0.1 |
| F-36 | Export folder configuration | v0.1 |
| F-37 | Default export format | v0.1 |
| F-38 | CPU thread count for inference | v0.1 |
| F-39 | Opt-in anonymous crash reports | v0.1 |
| F-40 | Check for updates toggle | v0.1 |
| F-41 | Keep model in memory toggle | v0.2 |

### 2.6 Recent transcripts

| ID | Requirement | Phase |
|---|---|---|
| F-42 | Persistent recent transcripts list | v0.1 |
| F-43 | Search / filter by filename | v0.1 |
| F-44 | Re-open past transcripts | v0.1 |
| F-45 | Clear history | v0.1 |

---

## 3. Non-functional requirements

### 3.1 Performance

| ID | Requirement | Target |
|---|---|---|
| NF-01 | Transcription speed on Standard tier CPU | RTFx ≥ 1.0 |
| NF-02 | App startup time | < 2 seconds |
| NF-03 | First segment displayed | < 5 seconds after start |
| NF-04 | UI responsiveness during transcription | No blocking — async pipeline |
| NF-05 | Installer size | < 50 MB (Whisper Tiny bundled) |
| NF-06 | Binary size (without models) | < 30 MB |

### 3.2 Compatibility

| ID | Requirement |
|---|---|
| NF-07 | Windows 10+ (64-bit) |
| NF-08 | macOS 12+ (Intel + Apple Silicon) |
| NF-09 | Linux (Ubuntu 22.04+, Fedora 38+) |
| NF-10 | Minimum 8 GB RAM (Whisper Tiny tier) |

### 3.3 Privacy

| ID | Requirement |
|---|---|
| NF-11 | No audio data sent over network during transcription |
| NF-12 | No transcript content in telemetry |
| NF-13 | Telemetry off by default |
| NF-14 | All model downloads verified with SHA256 |
| NF-15 | Settings stored locally (app_data_dir) |

### 3.4 Reliability

| ID | Requirement |
|---|---|
| NF-16 | App must not crash if model download interrupted |
| NF-17 | Partial downloads resumable on restart (v0.2) |
| NF-18 | Corrupted model detected and re-download offered |
| NF-19 | Transcription cancellable at any time |

---

## 4. Model registry

| Model | Size | Diarization | Min RAM | Phase |
|---|---|---|---|---|
| Whisper Tiny | 150 MB | No | 8 GB | v0.1 bundled |
| Whisper Base | 300 MB | No | 8 GB | v0.2 |
| Whisper Medium | 1.5 GB | No | 16 GB | v0.1 |
| Whisper Large v3 | 3.1 GB | No | 16 GB | v0.1 |
| VibeVoice INT4 | 5.2 GB | Yes | 8 GB | v0.2 |
| VibeVoice INT8 | 8.5 GB | Yes | 16 GB | v0.1 |
| VibeVoice FP16 | 14 GB | Yes | 32 GB | v0.3 |

---

## 5. Out of scope

- Cloud sync or remote storage
- Collaborative editing
- Real-time translation
- Mobile (iOS / Android)
- Browser extension
- Paid features or licensing enforcement