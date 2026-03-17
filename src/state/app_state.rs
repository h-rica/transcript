use leptos::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug)]
pub struct AppState {
    pub selected_file: RwSignal<Option<SelectedFile>>,
    pub selected_model: RwSignal<String>,
    pub selected_language: RwSignal<String>,
    pub active_model: RwSignal<String>,
    pub hardware_info: RwSignal<Option<HardwareInfo>>,
    pub settings: RwSignal<Settings>,
    pub audio_info: RwSignal<Option<AudioInfo>>,
    pub transcription_status: RwSignal<TranscriptionStatus>,
    pub transcription_progress: RwSignal<TranscriptionProgress>,
    pub transcript_segments: RwSignal<Vec<TranscriptSegment>>,
    pub transcription_summary: RwSignal<Option<TranscriptionSummary>>,
    pub error_message: RwSignal<Option<String>>,
}

pub fn provide_app_state() {
    let settings = Settings::default();
    let default_model = settings
        .default_model
        .clone()
        .unwrap_or_else(|| "whisper-tiny".into());

    let state = AppState {
        selected_file: RwSignal::new(None),
        selected_model: RwSignal::new(default_model.clone()),
        selected_language: RwSignal::new(settings.default_language.clone()),
        active_model: RwSignal::new(default_model),
        hardware_info: RwSignal::new(None),
        settings: RwSignal::new(settings),
        audio_info: RwSignal::new(None),
        transcription_status: RwSignal::new(TranscriptionStatus::Idle),
        transcription_progress: RwSignal::new(TranscriptionProgress::default()),
        transcript_segments: RwSignal::new(Vec::new()),
        transcription_summary: RwSignal::new(None),
        error_message: RwSignal::new(None),
    };
    provide_context(state);
}

pub fn use_app_state() -> AppState {
    use_context::<AppState>().expect("AppState not provided")
}

pub fn reset_transcription_state(state: &AppState) {
    state.audio_info.set(None);
    state.transcription_status.set(TranscriptionStatus::Idle);
    state
        .transcription_progress
        .set(TranscriptionProgress::default());
    state.transcript_segments.set(Vec::new());
    state.transcription_summary.set(None);
    state.error_message.set(None);
}
