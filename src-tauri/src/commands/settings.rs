use std::{fs, path::PathBuf};

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

fn settings_path() -> PathBuf {
    let base = dirs::config_dir()
        .or_else(dirs::data_local_dir)
        .unwrap_or_else(std::env::temp_dir);
    base.join("transcript").join("settings.toml")
}

#[tauri::command]
pub async fn get_settings() -> Result<Settings, String> {
    let path = settings_path();
    if !path.exists() {
        return Ok(Settings::default());
    }

    let contents = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    toml::from_str(&contents).map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn save_settings(settings: Settings) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let contents = toml::to_string_pretty(&settings).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_use_a_real_model() {
        let settings = Settings::default();
        assert_eq!(settings.default_model.as_deref(), Some("whisper-tiny"));
    }
}
