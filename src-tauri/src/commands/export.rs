#[tauri::command]
pub async fn export_transcript(_id: String, _format: String, _path: String) -> Result<(), String> {
    Ok(())
}
