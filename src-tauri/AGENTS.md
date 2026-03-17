# AGENTS.md

These instructions apply to files under `src-tauri/`.

Use the repo root `../AGENTS.md` for global rules and verification expectations. This file only adds backend-specific guidance.

## Backend Scope

The backend is the Tauri v2 crate that owns IPC commands, transcription orchestration, model management, exports, and hardware detection.

- IPC handlers live in `src/commands/`
- command registration happens in `src/lib.rs`
- ASR pipeline code lives in `src/asr/`
- audio decoding lives in `src/audio/`
- model logic lives in `src/models/`
- export logic lives in `src/export/`

## Backend Rules

- Every new IPC command must be registered in `src/lib.rs`.
- Keep command handlers thin when possible; place reusable logic in the appropriate domain module.
- Preserve the offline-first behavior. Do not add mandatory remote dependencies unless the user explicitly asks for them.
- Treat model downloads and large artifacts as operationally sensitive; avoid moving or renaming resource paths casually.
- Prefer explicit error paths over silent fallbacks in transcription, model, and export flows.

## Backend Change Checklist

- If you change a command contract, update the frontend caller in the same change.
- If you touch model registry or download behavior, confirm paths and filenames still align with `models/registry.toml` and `src-tauri/resources/`.
- If you touch ASR or audio code, avoid claiming end-to-end completeness unless the ONNX and decode path you changed is actually wired through.

## Backend Verification

When working only in `src-tauri/`, the fast local checks are:

- `cd src-tauri; cargo fmt`
- `cd src-tauri; cargo check`
- `cd src-tauri; cargo test`
- `cd src-tauri; cargo clippy -- -D warnings`

Before final handoff, still prefer the root `just fmt`, `just check`, `just lint`, and `just test` when the environment allows it.
