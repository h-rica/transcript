use serde::Serialize;
use tauri::AppHandle;

#[derive(Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub size_mb: u32,
    pub tier: String,
    pub bundled: bool,
    pub diarization: bool,
    pub languages: Vec<String>,
    pub status: String,
}

#[tauri::command]
pub async fn download_model(
    _app: AppHandle,
    _model_id: String,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn get_models() -> Result<Vec<ModelInfo>, String> {
    Ok(vec![])
}

#[tauri::command]
pub async fn delete_model(_model_id: String) -> Result<(), String> {
    Ok(())
}