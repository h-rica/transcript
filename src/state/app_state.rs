use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemePreference {
    #[default]
    Auto,
    Dark,
    Light,
}

impl ThemePreference {
    pub fn toggle(self) -> Self {
        match self {
            Self::Auto | Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedFile {
    pub path: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub ram_gb: u32,
    pub cpu_name: String,
    pub gpu_vram_gb: Option<u32>,
    pub tier: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub size_mb: u32,
    pub tier: String,
    pub bundled: bool,
    pub diarization: bool,
    pub languages: Vec<String>,
    pub source: String,
    pub status: String,
    pub rtfx: f32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub default_language: String,
    pub default_model: Option<String>,
    pub cpu_threads: u32,
    pub keep_model_in_memory: bool,
    pub export_path: String,
    pub default_export_format: String,
    pub include_timestamps: bool,
    pub include_speaker_labels: bool,
    pub telemetry: bool,
    pub check_for_updates: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_language: "fr".into(),
            default_model: Some("whisper-tiny".into()),
            cpu_threads: 4,
            keep_model_in_memory: false,
            export_path: String::new(),
            default_export_format: "txt".into(),
            include_timestamps: true,
            include_speaker_labels: true,
            telemetry: false,
            check_for_updates: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioInfo {
    pub duration_s: f32,
    pub size_bytes: u64,
    pub format: String,
    pub bitrate_kbps: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub speaker: String,
    pub text: String,
    pub start_s: f32,
    pub end_s: f32,
    pub language: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionProgress {
    pub percent: f32,
    pub elapsed_s: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionSummary {
    pub segments: u32,
    pub speakers: u32,
    pub words: u32,
    pub language: String,
    pub elapsed_s: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TranscriptionStatus {
    Idle,
    LoadingModel,
    Running,
    Complete,
    Failed(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiError {
    pub message: String,
}

impl UiError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl From<String> for UiError {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

impl From<&str> for UiError {
    fn from(message: &str) -> Self {
        Self::new(message)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TranscriptionRequest {
    pub path: String,
    pub model_id: String,
    pub language: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExportRequest {
    pub id: String,
    pub format: String,
    pub path: String,
}

#[derive(Clone, Debug)]
pub struct AppShellState {
    pub theme_preference: RwSignal<ThemePreference>,
    pub selected_file: RwSignal<Option<SelectedFile>>,
    pub selected_model: RwSignal<String>,
    pub selected_language: RwSignal<String>,
    pub active_model: RwSignal<String>,
    pub hardware_info: RwSignal<Option<HardwareInfo>>,
    pub available_models: RwSignal<Vec<WorkspaceModel>>,
    pub settings: RwSignal<Settings>,
}

#[derive(Clone, Debug)]
pub struct TranscriptionSessionState {
    pub audio_info: RwSignal<Option<AudioInfo>>,
    pub status: RwSignal<TranscriptionStatus>,
    pub progress: RwSignal<TranscriptionProgress>,
    pub segments: RwSignal<Vec<TranscriptSegment>>,
    pub summary: RwSignal<Option<TranscriptionSummary>>,
    pub error: RwSignal<Option<String>>,
    pub listeners_ready: RwSignal<bool>,
}

#[derive(Clone, Debug)]
pub struct TranscriptViewState {
    pub active_tab: RwSignal<String>,
    pub speaker_filter: RwSignal<String>,
    pub export_open: RwSignal<bool>,
    pub export_format: RwSignal<String>,
    pub export_message: RwSignal<Option<String>>,
}

pub fn provide_app_state() {
    let settings = Settings::default();
    let default_model = settings
        .default_model
        .clone()
        .unwrap_or_else(|| "whisper-tiny".into());

    provide_context(AppShellState {
        theme_preference: RwSignal::new(ThemePreference::Dark),
        selected_file: RwSignal::new(None),
        selected_model: RwSignal::new(default_model.clone()),
        selected_language: RwSignal::new(settings.default_language.clone()),
        active_model: RwSignal::new(default_model),
        hardware_info: RwSignal::new(None),
        available_models: RwSignal::new(Vec::new()),
        settings: RwSignal::new(settings.clone()),
    });

    provide_context(TranscriptionSessionState {
        audio_info: RwSignal::new(None),
        status: RwSignal::new(TranscriptionStatus::Idle),
        progress: RwSignal::new(TranscriptionProgress::default()),
        segments: RwSignal::new(Vec::new()),
        summary: RwSignal::new(None),
        error: RwSignal::new(None),
        listeners_ready: RwSignal::new(false),
    });

    provide_context(TranscriptViewState {
        active_tab: RwSignal::new("speakers".into()),
        speaker_filter: RwSignal::new(String::new()),
        export_open: RwSignal::new(false),
        export_format: RwSignal::new(settings.default_export_format),
        export_message: RwSignal::new(None),
    });
}

pub fn use_app_shell_state() -> AppShellState {
    use_context::<AppShellState>().expect("AppShellState not provided")
}

pub fn use_transcription_session_state() -> TranscriptionSessionState {
    use_context::<TranscriptionSessionState>().expect("TranscriptionSessionState not provided")
}

pub fn use_transcript_view_state() -> TranscriptViewState {
    use_context::<TranscriptViewState>().expect("TranscriptViewState not provided")
}

pub fn reset_transcription_session(session: &TranscriptionSessionState) {
    session.audio_info.set(None);
    session.status.set(TranscriptionStatus::Idle);
    session.progress.set(TranscriptionProgress::default());
    session.segments.set(Vec::new());
    session.summary.set(None);
    session.error.set(None);
}

pub fn reset_transcript_view(view: &TranscriptViewState, export_format: String) {
    view.active_tab.set("speakers".into());
    view.speaker_filter.set(String::new());
    view.export_open.set(false);
    view.export_format.set(export_format);
    view.export_message.set(None);
}
