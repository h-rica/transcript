use std::{fs, path::Path, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreference {
    #[default]
    Auto,
    Dark,
    Light,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Settings {
    pub theme_preference: ThemePreference,
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
            theme_preference: ThemePreference::Auto,
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

fn load_settings_from_path(path: &Path) -> Result<Settings, String> {
    if !path.exists() {
        return Ok(Settings::default());
    }

    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    toml::from_str(&contents).map_err(|error| error.to_string())
}

fn save_settings_to_path(path: &Path, settings: &Settings) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let contents = toml::to_string_pretty(settings).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_settings() -> Result<Settings, String> {
    load_settings_from_path(&settings_path())
}

#[tauri::command]
pub async fn save_settings(settings: Settings) -> Result<(), String> {
    save_settings_to_path(&settings_path(), &settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_use_a_real_model() {
        let settings = Settings::default();
        assert_eq!(settings.theme_preference, ThemePreference::Auto);
        assert_eq!(settings.default_model.as_deref(), Some("whisper-tiny"));
    }

    #[test]
    fn settings_roundtrip_preserves_theme_preference() {
        let unique = format!(
            "transcript-settings-{}.toml",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let path = std::env::temp_dir().join(unique);
        let settings = Settings {
            theme_preference: ThemePreference::Dark,
            default_language: "en".into(),
            default_model: Some("whisper-medium".into()),
            cpu_threads: 6,
            keep_model_in_memory: true,
            export_path: "C:/Exports".into(),
            default_export_format: "srt".into(),
            include_timestamps: true,
            include_speaker_labels: false,
            telemetry: true,
            check_for_updates: false,
        };

        save_settings_to_path(&path, &settings).unwrap();
        let loaded = load_settings_from_path(&path).unwrap();
        assert_eq!(loaded.theme_preference, ThemePreference::Dark);
        assert_eq!(loaded.default_language, "en");
        assert_eq!(loaded.default_model.as_deref(), Some("whisper-medium"));
        let _ = fs::remove_file(path);
    }
}
