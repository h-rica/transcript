# AGENTS.md

These instructions apply to files under `src/`.

Use the repo root `../AGENTS.md` for global rules and verification expectations. This file only adds frontend-specific guidance.

## Frontend Scope

The frontend is a Leptos 0.8 CSR app.

- route-level screens live in `src/pages/`
- reusable UI belongs in `src/components/`
- cross-route state belongs in `src/state/app_state.rs`
- module exports are organized through `src/pages/mod.rs`, `src/components/mod.rs`, and `src/state/mod.rs`

## Frontend Rules

- Match existing Leptos patterns before introducing new abstractions.
- Keep state local unless it is genuinely shared across routes or major UI sections.
- When adding a page, update both routing and the relevant module exports.
- When adding a reusable component, wire it through `src/components/mod.rs`.
- Do not introduce a JS or TS frontend sidecar for UI work in this repo.
- Prefer code that is straightforward to trace over clever reactive indirection.

## Frontend Change Checklist

- If the UI calls a new backend capability, update the matching Tauri command in the same change.
- Keep the offline-first desktop workflow intact; avoid assumptions about remote services.
- If a screen depends on partially implemented backend behavior, document that constraint in the handoff instead of masking it with fake success states.

## Frontend Verification

When working only in `src/`, the fast local checks are:

- `cargo fmt`
- `cargo check`
- `cargo test`
- `cargo clippy -- -D warnings`

Before final handoff, still prefer the root `just fmt`, `just check`, `just lint`, and `just test` when the environment allows it.
