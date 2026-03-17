# Transcript - User Flow

**Version**: 0.2.0  
**Updated**: March 2026  
**Scope**: POC v0.1

---

## 1. Happy path - first launch

```
+-------------------------------------------------------------+
|                         FIRST LAUNCH                        |
|                                                             |
|  App detects hardware tier                                  |
|       |                                                     |
|       v                                                     |
|  "Whisper Tiny is ready to use."                            |
|  "Your machine can also run VibeVoice INT8 for diarization."|
|  [Download VibeVoice INT8 - 8.5 GB]  [Continue with Tiny]   |
|                                                             |
+-------------------------------------------------------------+
```

This first-launch path should recommend VibeVoice INT8 when hardware allows it, but it should not block the user from starting with Whisper Tiny immediately.

---

## 2. Main flow - transcribe a file

```
START
  |
  v
+------------------------------+
| HOME SCREEN                  |
|                              |
|  Drop audio file here        |
|  or click to browse          |
|                              |
|  Recent transcripts          |
+--------------+---------------+
               | file selected
               v
+------------------------------+
| FILE PREVIEW                 |
|                              |
|  filename.mp3  •  00:24:38   |
|                              |
|  Language:  [FR / EN]        |
|  Model:     [VibeVoice INT8] |
|             [Change model]   |
|                              |
|  [Start Transcription]       |
+--------------+---------------+
               | user clicks Start
               v
+------------------------------+
| TRANSCRIPTION IN PROGRESS    |
|                              |
|  Progress 64%                |
|  Elapsed 00:01:23            |
|  Speed 1.3x real-time        |
|                              |
|  Live segments               |
|  Speaker A ...               |
|  Speaker B ...               |
|                              |
|  [Cancel]                    |
+--------------+---------------+
               | transcription complete
               v
+------------------------------+
| TRANSCRIPT VIEW              |
|                              |
|  Speakers | Timeline | Raw   |
|                              |
|  Segment list                |
|                              |
|  [Export]  [Copy all]        |
+--------------+---------------+
               | user clicks Export
               v
+------------------------------+
| EXPORT                       |
|                              |
|  Format: TXT or SRT          |
|  Location: ~/Documents/...   |
|                              |
|  [Save]                      |
+--------------+---------------+
               | file saved
               v
              END
```

---

## 3. Model download flow

Triggered from first launch, File Preview, or Model Manager.

```
[Change model]
      |
      v
+--------------------------------------------------+
| MODEL SELECTOR                                   |
|                                                  |
| Recommended for your hardware:                   |
|                                                  |
| • VibeVoice INT8   8.5 GB   Diarization yes      |
| • Whisper Tiny      150 MB  Bundled              |
|                                                  |
| [Download VibeVoice INT8]  [Use Whisper Tiny]    |
+-------------------+------------------------------+
                    | user clicks Download
                    v
+------------------------------+
| DOWNLOADING                  |
|                              |
|  VibeVoice INT8              |
|  4.2 / 8.5 GB                |
|  12.3 MB/s  ETA 03:24        |
|                              |
|  SHA256 verification -> pass |
|                              |
|  [Cancel]                    |
+-------------------+----------+
                    | complete
                    v
         Model ready -> return to File Preview
```

For v0.1, interrupted downloads may fail and require a restart of the download. Resume support is planned for v0.2, not part of the POC contract.

---

## 4. Error states

| State | UI | Action |
| --- | --- | --- |
| Unsupported file format | Toast: "Format not supported. Use MP3, WAV, or M4A." | Dismiss |
| Not enough RAM for selected model | Warning banner with lighter-model suggestion | Offer switch to Whisper Tiny |
| Download interrupted | Error message + retry action | Restart download |
| Model file corrupted (SHA256 fail) | "Verification failed" + re-download action | Delete partial files and retry |
| Transcription failed | Error message + logs accessible | Retry |

---

## 5. Screen inventory (POC v0.1)

| Screen | Route | Description |
| --- | --- | --- |
| Home | `/` | Drop zone + recent files |
| File Preview | `/preview` | File info + model/language selection |
| Transcription | `/transcription` | Live progress + streaming segments |
| Transcript View | `/transcript/:id` | Full transcript with speakers + timeline |
| Model Manager | `/models` | Download, delete, storage usage |
| Settings | `/settings` | Language default, export path, telemetry |

---

## 6. UI states per screen

### Home

- `empty` - no recent transcripts, drop zone prominent
- `has_recents` - recent files listed below drop zone
- `dragging` - drop zone highlighted on file drag

### File Preview

- `ready` - selected model available and start button enabled
- `low_ram_warning` - recommended model exceeds comfortable hardware budget
- `model_not_downloaded` - selected model missing locally, download action shown
- `model_selector_open` - alternate models visible

### Transcription

- `loading` - model loading into memory
- `running` - progress bar + live segments streaming
- `cancelled` - return to Home
- `complete` - navigate to Transcript View after success state

### Model Manager

- `not_downloaded` - download button + size indicator
- `downloading` - progress bar + cancel action
- `ready` - use + delete actions
- `selected` - currently active model

---

## 7. Product rules clarified by this flow

- v0.1 uses manual language selection only. The UI should show `FR` and `EN`, not `Auto`.
- Full multilingual auto-detection is a later feature and should not appear as an active control in the POC.
- The concrete model catalog for the POC is `Whisper Tiny` plus `VibeVoice INT8`.
- Any future variants such as `VibeVoice INT4` or `VibeVoice FP16` should be added only after they exist as real downloadable artifacts.
