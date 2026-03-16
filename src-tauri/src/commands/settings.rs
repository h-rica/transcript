use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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
            default_model: None,
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

#[tauri::command]
pub async fn get_settings() -> Result<Settings, String> {
    Ok(Settings::default())
}

#[tauri::command]
pub async fn save_settings(_settings: Settings) -> Result<(), String> {
    Ok(())
}