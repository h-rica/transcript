# AGENTS.md

This is the canonical instruction file for coding agents working in this repository.

Read `./MEMORY.md` at the start of every non-trivial task. It captures durable project memory, recurring blockers, and prevention rules that should reduce repeated mistakes and wasted time.

Use the nearest `AGENTS.md` in the directory tree for file-specific rules:

- `./AGENTS.md`: repo-wide rules
- `./src/AGENTS.md`: frontend-only rules
- `./src-tauri/AGENTS.md`: backend-only rules

If this file conflicts with an explicit user instruction, the user instruction wins.

## Project Memory

- `./MEMORY.md`: durable agent memory for recurring blockers, workflow defaults, and prevention notes
- use `MEMORY.md` for stable lessons that should affect future tasks
- use `docs/tasks.md` for active roadmap items and temporary task tracking

## Project Snapshot

This repository is an offline-first desktop transcription app built with:

- Frontend: Rust, `leptos = 0.8`, `leptos_router = 0.8`, `singlestage`
- Desktop shell / IPC: Tauri v2
- Backend: Rust 2024 edition
- Audio decode: `symphonia`
- Whisper fallback: `whisper-rs`
- ONNX runtime: `ort = 2.0.0-rc.12`

Treat `Cargo.toml`, `src-tauri/Cargo.toml`, and the current code as the source of truth when docs drift.

## Repository Map

- `src/`: Leptos frontend
- `src/app.rs`: router and top-level app shell
- `src/state/app_state.rs`: shared reactive app state
- `src/pages/`: route-level screens
- `src/components/`: reusable UI pieces
- `src-tauri/src/lib.rs`: Tauri app entry and command registration
- `src-tauri/src/commands/`: Tauri IPC handlers
- `src-tauri/src/asr/`: transcription pipeline pieces
- `src-tauri/src/audio/`: audio decoding
- `src-tauri/src/models/`: model registry, downloads, hardware logic
- `src-tauri/src/export/`: TXT and SRT export
- `models/registry.toml`: model catalog
- `docs/tasks.md`: best snapshot of current work and placeholders
- `justfile`: preferred task runner commands

## Preferred Commands

Prefer `just` recipes over raw cargo commands:

- `just dev`: run the Tauri app in development
- `just check`: run frontend and backend `cargo check`
- `just test`: run frontend and backend tests
- `just fmt`: format all Rust code
- `just lint`: run clippy with warnings denied

Useful one-off setup commands:

- `just download-onnx`: fetch large ONNX artifacts into `src-tauri/resources/onnx`
- `just download-whisper-tiny`: fetch the fallback Whisper model

This repo is effectively two Rust crates:

- root crate: frontend WASM app
- `src-tauri/`: backend Tauri crate

## Working Rules

- Prefer small, targeted changes over broad refactors unless the task explicitly asks for restructuring.
- Preserve the Rust-first architecture. Do not add a JS or TS sidecar unless the user explicitly asks for one.
- Prefer updating existing modules over creating parallel abstractions.
- When a feature crosses the frontend/backend boundary, update both sides in the same change.
- Prefer actual code and manifests over roadmap docs if they disagree.
- Keep docs edits narrow unless the task is documentation-focused.

## Reality Checks

Do not assume every planned feature already exists. Based on the current implementation:

- routing exists for home, preview, transcription, transcript view, models, and settings
- shared app state covers selected file, selected model, hardware info, and active model
- several backend and UI modules are still placeholders or partial implementations
- the audio decoder still needs fixed 24 kHz resampling
- transcription cancellation is registered but not fully implemented
- the ONNX path is partially wired; the end-to-end decoder flow is not complete

## Areas To Treat Carefully

- `src-tauri/resources/onnx/`: large model artifacts; do not rewrite or move them unless the task is explicitly about model assets
- `target/`: build output, not source
- `src-tauri/icons/`: generated app icons; avoid incidental edits
- `docs/tasks.md`: may already contain in-progress edits; do not overwrite unrelated work

## Change Hygiene

- Keep new docs and code ASCII unless the file already uses non-ASCII.
- Do not touch unrelated dirty files.
- Avoid editing planning docs just to match code unless the task is documentation-focused.
- When adding TODOs, make them specific and actionable.

## Good Starting Points By Task

- New page or UX flow: `src/app.rs`, `src/pages/`, `src/components/`, `src/state/app_state.rs`
- New IPC command: `src-tauri/src/commands/`, then register in `src-tauri/src/lib.rs`
- Model logic: `models/registry.toml`, `src-tauri/src/models/`
- Export behavior: `src-tauri/src/export/` and `src-tauri/src/commands/export.rs`
- Transcription pipeline: `src-tauri/src/asr/`, `src-tauri/src/audio/`, `src-tauri/src/commands/transcribe.rs`

## Verification

Default verification bar for code changes:

- `just fmt`
- `just check`
- `just lint`
- `just test`

Use narrower commands only for iteration. Before declaring non-doc work complete, either run the full bar or explain clearly why you could not.

## Definition Of Done

A change is ready when:

- the requested behavior is implemented end to end, not just scaffolded
- frontend/backend wiring is complete when IPC is involved
- no placeholder logic remains on the delivered path unless the user explicitly accepted a stub
- the relevant verification passes, or any blocked verification is called out explicitly
- unrelated files are left untouched
- the final handoff explains what changed, what was verified, and any remaining gap
