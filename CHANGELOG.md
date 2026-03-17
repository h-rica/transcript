# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-03-17

Initial pre-release foundation for the offline desktop transcription app, developed across 2026-03-16 and 2026-03-17.

### Added
- Offline-first desktop application foundation built with Tauri v2 and a Leptos-based Rust frontend.
- Core backend module structure and Tauri command stubs for transcription workflows.
- Symphonia-based audio decoding pipeline for MP3, WAV, and M4A inputs.
- Whisper-based transcription integration and streamed `transcribe_file` events for incremental results in the desktop app.
- ONNX-based ASR pipeline and model routing to support local model execution beyond the initial Whisper path.
- Whisper Tiny bundled-resource support plus task recipes for fetching Whisper Tiny and VibeVoice ONNX artifacts.
- Home page UI with router setup, shared app state, file drop zone, sidebar navigation, and recent file list.

### Changed
- Expanded the README with setup, build, architecture, and model guidance.
- Added Tailwind CSS v4, Singlestage UI, and `just` automation to support frontend and desktop development.
- Improved developer workflows with cross-platform cleanup tasks, build checks, lint runs, and model-file ignore rules.

### Documentation
- Added project planning notes, model bundling guidance, and UI wireframe documentation.
- Documented Windows-specific prerequisites, including LLVM/libclang and the Visual Studio C++ workload needed for local builds.

### Timeline

#### 2026-03-17
- Added the ONNX pipeline path and model routing for `transcribe_file`.
- Added a `just` recipe for downloading VibeVoice ONNX artifacts.
- Added build-check and lint automation to tighten local verification.
- Added the Leptos router, shared app state context, and page stubs.
- Added the Home page UI with a drop zone, sidebar, and recent-files view.
- Improved developer cleanup and repository ignore rules.
- Expanded planning and wireframe documentation.

#### 2026-03-16
- Created the initial repository scaffold and expanded the project README.
- Added Tailwind CSS v4, Singlestage UI, and the initial `justfile`.
- Added backend command stubs and the application module structure.
- Implemented Symphonia-based decoding for MP3, WAV, and M4A audio.
- Added the Whisper transcription path, fallback dependency, and streamed Tauri transcription events.
- Added model bundling support for Whisper Tiny and recipes for fetching model assets.
- Documented project planning, model bundling, LLVM/libclang, and Visual Studio C++ prerequisites.

[Unreleased]: https://github.com/h-rica/transcript/compare/2644cf7...HEAD
[0.1.0]: https://github.com/h-rica/transcript/compare/2391ba5...2644cf7
