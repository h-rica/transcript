# MEMORY.md

This file is the project's durable working memory for agents and contributors.

Its purpose is to improve execution speed, reduce repeated mistakes, and make blocker handling consistent across tasks.

If guidance here conflicts with a direct user request or `AGENTS.md`, follow the user request first, then `AGENTS.md`.

## How To Use This File

- Read this file together with the nearest `AGENTS.md` before starting any non-trivial task.
- Update it only when you learn something durable that should help future tasks.
- Keep it short, actionable, and easy to scan.
- Do not store temporary status, plans, or branch-specific notes here; use `docs/tasks.md` for active work tracking.

## Task Start Defaults

- Confirm the relevant files and current implementation before proposing or making changes.
- Check whether the work lives in `src/` or `src-tauri/` and follow the matching subtree rules.
- Prefer current code, manifests, and registered commands over roadmap documents when they disagree.
- Keep changes narrow and complete on the delivered path instead of scattering partial scaffolding.
- Treat frontend and backend contracts as a single change when a task crosses IPC or shared data boundaries.

## Stable Project Realities

- This is an offline-first desktop app; avoid introducing mandatory online dependencies unless explicitly requested.
- The repository is effectively two Rust crates: the root frontend crate and the `src-tauri/` backend crate.
- Several UI and backend areas are still partial; verify actual wiring before extending existing placeholders.
- The audio pipeline still needs careful validation around fixed 24 kHz behavior.
- Transcription cancellation is registered but not fully complete end to end.
- The ONNX path is only partially wired; do not describe placeholder decoder behavior as finished transcription.

## Recurring Prevention Rules

- Prefer updating existing modules over creating parallel abstractions.
- Do not move or rewrite large model artifacts, generated icons, or build output unless the task explicitly targets them.
- When touching model or asset behavior, re-check `models/registry.toml`, `src-tauri/resources/`, and related command wiring.
- Leave unrelated dirty files untouched.
- Add documentation only when it captures a durable rule, known pitfall, or repeat blocker.
- Prefer `just` recipes for repo-wide verification when the environment allows it.

## Blocker Handling

1. Verify the blocker in code, config, or command output before reporting it.
2. Classify it clearly: missing implementation, broken wiring, environment issue, asset issue, or unclear requirement.
3. Prefer the smallest honest unblock that preserves end-to-end correctness.
4. If delivery is blocked, state what is blocked, why it is blocked, what was verified, and the next concrete step.
5. Do not hide blockers behind fake success states, silent fallbacks, or TODO-only scaffolding unless the user explicitly accepts that tradeoff.

## Task Closeout Defaults

- State what changed.
- State what was verified, or why verification could not be completed.
- Call out any remaining gaps only when they still matter for the delivered path.

## Good Memory Update Test

A new note belongs here only if it is:

- short enough to scan in under a minute
- specific enough to change future behavior
- durable enough to matter beyond the current task
