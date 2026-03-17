use serde::Serialize;
use tauri::AppHandle;

use crate::models::registry::{load_registry, model_is_installed};

#[derive(Clone, Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub size_mb: u32,
    pub tier: String,
    pub bundled: bool,
    pub diarization: bool,
    pub languages: Vec<String>,
    pub source: String,
    pub status: String,
}

#[tauri::command]
pub async fn download_model(_app: AppHandle, _model_id: String) -> Result<(), String> {
    Err("Model download is not implemented in this build yet.".into())
}

#[tauri::command]
pub async fn get_models() -> Result<Vec<ModelInfo>, String> {
    load_registry()
        .map(|models| {
            models
                .into_iter()
                .map(|model| {
                    let bundled = model.bundled;
                    let status = if bundled {
                        "bundled".to_string()
                    } else if model_is_installed(&model) {
                        "downloaded".to_string()
                    } else {
                        "missing".to_string()
                    };
                    ModelInfo {
                        id: model.id,
                        name: model.name,
                        size_mb: model.size_mb,
                        tier: model.tier,
                        bundled,
                        diarization: model.diarization,
                        languages: model.languages,
                        source: model.source,
                        status,
                    }
                })
                .collect()
        })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn delete_model(_model_id: String) -> Result<(), String> {
    Err("Model deletion is not implemented in this build yet.".into())
}
