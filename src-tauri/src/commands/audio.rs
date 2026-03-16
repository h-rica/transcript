use serde::Serialize;

#[derive(Serialize)]
pub struct AudioInfo {
    pub duration_s: f32,
    pub size_bytes: u64,
    pub format: String,
    pub bitrate_kbps: Option<u32>,
}

#[tauri::command]
pub async fn get_audio_info(_path: String) -> Result<AudioInfo, String> {
    Err("Not implemented".into())
}