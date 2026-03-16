use tauri::AppHandle;

#[tauri::command]
pub async fn transcribe_file(
    _app: AppHandle,
    _path: String,
    _model_id: String,
    _language: String,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn cancel_transcription() -> Result<(), String> {
    Ok(())
}