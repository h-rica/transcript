# Transcript — User Flow

**Version** : 0.2.0
**Updated** : March 2026
**Scope** : POC v0.1

---

## 1. Happy path — first launch

```
┌─────────────────────────────────────────────────────────────┐
│  FIRST LAUNCH                                               │
│                                                             │
│  App detects hardware tier                                  │
│       │                                                     │
│       ▼                                                     │
│  "Whisper Tiny is bundled and ready."                       │
│  "Your machine supports VibeVoice INT8 (diarization)."      │
│  [Download VibeVoice INT8 — 8.5 GB]  [Continue with Tiny]  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Main flow — transcribe a file

```
START
  │
  ▼
┌──────────────────────────────┐
│  HOME SCREEN                 │
│                              │
│  ┌────────────────────────┐  │
│  │   Drop audio file here │  │
│  │   or click to browse   │  │
│  └────────────────────────┘  │
│                              │
│  Recent transcripts ▾        │
└──────────────┬───────────────┘
               │  File dropped / selected
               ▼
┌──────────────────────────────┐
│  FILE PREVIEW                │
│                              │
│  filename.mp3  •  00:24:38   │
│                              │
│  Language:  [Auto / FR / EN] │
│  Model:     [VibeVoice INT8] │  ← hardware-aware suggestion
│             [Change model ▾] │
│                              │
│  [Start Transcription]       │
└──────────────┬───────────────┘
               │  User clicks Start
               ▼
┌──────────────────────────────┐
│  TRANSCRIPTION IN PROGRESS   │
│                              │
│  ████████████░░░░░░  64%     │
│  Elapsed: 00:01:23           │
│  Speed: 1.3× real-time       │
│                              │
│  Live segments:              │
│  ┌────────────────────────┐  │
│  │ 🔵 Speaker A  00:00:04 │  │
│  │ "Bonjour, bienvenue..." │  │
│  │                        │  │
│  │ 🟠 Speaker B  00:00:18 │  │
│  │ "Merci de nous..."     │  │
│  └────────────────────────┘  │
│                              │
│  [Cancel]                    │
└──────────────┬───────────────┘
               │  Transcription complete
               ▼
┌──────────────────────────────┐
│  TRANSCRIPT VIEW             │
│                              │
│  Speakers  Timeline  Raw     │  ← tabs
│                              │
│  🔵 Speaker A                │
│  00:00:04 → 00:00:17         │
│  "Bonjour, bienvenue à..."   │
│                              │
│  🟠 Speaker B                │
│  00:00:18 → 00:00:31         │
│  "Merci de nous rejoindre."  │
│                              │
│  [Export ▾]  [Copy all]      │
└──────────────┬───────────────┘
               │  User clicks Export
               ▼
┌──────────────────────────────┐
│  EXPORT                      │
│                              │
│  Format:  ● TXT  ○ SRT       │
│  Location: ~/Documents/      │
│                              │
│  [Save]                      │
└──────────────────────────────┘
               │  File saved
               ▼
             END
```

---

## 3. Model download flow

Triggered from the model selector or the first-launch screen.

```
[Change model ▾]
       │
       ▼
┌──────────────────────────────────────────────────────┐
│  MODEL SELECTOR                                      │
│                                                      │
│  Recommended for your hardware (32 GB RAM):          │
│                                                      │
│  ● VibeVoice INT8    8.5 GB   Diarization ✓         │
│  ○ Whisper Large v3  3.1 GB   Diarization ✗         │
│  ○ Whisper Medium    1.5 GB   Diarization ✗         │
│  ○ Whisper Tiny       150 MB  Bundled ✓ Diarization ✗│
│                                                      │
│  [Download VibeVoice INT8]  [Use Whisper Tiny]       │
└──────────────┬───────────────────────────────────────┘
               │  User clicks Download
               ▼
┌──────────────────────────────┐
│  DOWNLOADING                 │
│                              │
│  VibeVoice INT8              │
│  ████████░░░░░░░░  4.2/8.5GB │
│  12.3 MB/s  •  ETA 03:24     │
│                              │
│  SHA256 verification... ✓    │
│                              │
│  [Cancel]                    │
└──────────────┬───────────────┘
               │  Download + verification complete
               ▼
         Model ready — return to file preview
```

---

## 4. Error states

| State | UI | Action |
|---|---|---|
| Unsupported file format | Toast "Format not supported. Use MP3, WAV, or M4A." | — |
| Not enough RAM for selected model | Warning banner with alternative suggestion | Offer to switch to lighter model |
| Download interrupted | "Download paused" + [Resume] button | Resumable download |
| Model file corrupted (SHA256 fail) | "Verification failed" + [Re-download] | Delete + re-download |
| Transcription failed | Error message + logs accessible | [Retry] button |

---

## 5. Screen inventory (POC v0.1)

| Screen | Route | Description |
|---|---|---|
| Home | `/` | Drop zone + recent files |
| File Preview | `/preview` | File info + model/language selection |
| Transcription | `/transcription` | Live progress + streaming segments |
| Transcript View | `/transcript/:id` | Full transcript with speakers + timeline |
| Model Manager | `/models` | Download, delete, storage usage |
| Settings | `/settings` | Language default, export path, telemetry |

---

## 6. UI states per screen

### Home
- `empty` — No recent transcripts, drop zone prominent
- `has_recents` — Recent files listed below drop zone
- `dragging` — Drop zone highlighted on file drag

### Transcription
- `loading` — Model loading into memory
- `running` — Progress bar + live segments streaming
- `paused` — Not implemented in v0.1
- `cancelled` — Return to Home
- `complete` — Auto-navigate to Transcript View

### Model Manager
- `not_downloaded` — [Download] button + size indicator
- `downloading` — Progress bar + [Cancel]
- `ready` — [Use] + [Delete] actions
- `selected` — Checkmark, currently active model