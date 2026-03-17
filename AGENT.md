# AGENT.md

## Purpose

This repository is an offline-first desktop transcription app built with Tauri v2, a Rust/Leptos frontend, and a Rust backend for audio decoding, ASR orchestration, model management, and export.

Use this file as the working agreement for code changes in this repo. Keep changes aligned with the current implementation, not older planning docs.

## Current stack

- Frontend: `leptos = 0.8`, `leptos_router = 0.8`, `singlestage`
- Desktop shell / IPC: `tauri = 2`
- Backend runtime: Rust 2024 edition
- Audio decode: `symphonia`
- Whisper fallback: `whisper-rs`
- ONNX runtime: `ort = 2.0.0-rc.12`

Note: some docs still mention Leptos 0.7. The `Cargo.toml` files are the source of truth.

## Repository map

- `src/`: Leptos frontend
- `src/app.rs`: router and top-level app shell
- `src/state/app_state.rs`: shared reactive app state
- `src/pages/`: route-level screens
- `src/components/`: reusable UI pieces
- `src-tauri/src/lib.rs`: Tauri app entry, command registration
- `src-tauri/src/commands/`: Tauri IPC handlers
- `src-tauri/src/asr/`: transcription pipeline pieces
- `src-tauri/src/audio/`: audio decoding
- `src-tauri/src/models/`: model registry, download, hardware logic
- `src-tauri/src/export/`: TXT and SRT export
- `models/registry.toml`: model catalog
- `docs/tasks.md`: best snapshot of in-progress vs placeholder work
- `justfile`: preferred task runner commands

## Preferred commands

Use `just` recipes when possible:

- `just dev`: run the Tauri app in development
- `just check`: run frontend and backend `cargo check`
- `just test`: run frontend and backend tests
- `just fmt`: format all Rust code
- `just lint`: run clippy with warnings denied

If using raw cargo commands, remember this is effectively a two-crate repo:

- Root crate: frontend WASM app
- `src-tauri/`: backend Tauri crate

## Working rules

- Prefer small, targeted changes over broad refactors unless the task explicitly calls for restructuring.
- Preserve the Rust-first architecture. Do not add a JS/TS sidecar unless the user asks for it.
- Match existing Leptos patterns: route components in `src/pages`, app-wide state in `src/state`, reusable UI in `src/components`.
- Backend commands must be wired through `src-tauri/src/lib.rs`.
- When adding a new feature that crosses the boundary, update both the command handler and the frontend call site in the same change.
- Prefer the actual code over roadmap docs if they disagree.

## Files and areas to treat carefully

- `src-tauri/resources/onnx/`: large model artifacts; do not rewrite or move them unless the task is explicitly about model assets.
- `target/`: build output, not source.
- `src-tauri/icons/`: generated app icons; avoid incidental edits.
- `docs/tasks.md`: this may already have user edits in progress; do not overwrite unrelated changes.

## Current implementation reality

Based on the current code and task list:

- Routing exists for home, preview, transcription, transcript view, models, and settings.
- Shared app state currently covers selected file, selected model, hardware info, and active model.
- Several backend and UI modules are still placeholders or partial implementations.
- The audio decoder still needs fixed 24 kHz resampling.
- Transcription cancellation is registered but not fully implemented.
- The ONNX path is partially wired; the end-to-end decoder flow is not complete.

Do not assume roadmap items are already implemented just because routes or modules exist.

## Verification expectations

Default to full-project verification before declaring work complete:

- `just fmt`
- `just check`
- `just lint`
- `just test`

Use narrower commands only for quick iteration while working, not as the final verification bar, unless the task is strictly limited to documentation or the environment makes full verification impossible.

If you cannot run full verification, say so explicitly, explain why, and treat the change as not fully done.

## Change hygiene

- Keep new docs and code ASCII unless the file already uses non-ASCII.
- Do not touch unrelated dirty files.
- Avoid editing planning docs just to match code unless the task is documentation-focused.
- Prefer updating existing modules over creating parallel abstractions.
- When adding TODOs, make them specific and actionable.

## Good starting points by task

- New page or UX flow: `src/app.rs`, `src/pages/`, `src/components/`, `src/state/app_state.rs`
- New IPC command: `src-tauri/src/commands/`, then register in `src-tauri/src/lib.rs`
- Model logic: `models/registry.toml`, `src-tauri/src/models/`
- Export behavior: `src-tauri/src/export/` and `src-tauri/src/commands/export.rs`
- Transcription pipeline: `src-tauri/src/asr/`, `src-tauri/src/audio/`, `src-tauri/src/commands/transcribe.rs`

## Definition of done

A change is ready when:

- the requested behavior is implemented end to end, not just scaffolded,
- the result matches the expected feature behavior from the user request and the current roadmap or task docs,
- command wiring is complete if IPC is involved,
- no placeholder logic remains for the code path being delivered unless the user explicitly accepted a stub,
- `just fmt`, `just check`, `just lint`, and `just test` all pass green, unless the task is documentation-only or the environment blocks execution,
- any blocked verification is called out explicitly as an outstanding gap, not silently treated as complete,
- unrelated files are left untouched,
- and the final note explains what changed, what was verified, and any remaining gap.
