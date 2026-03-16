use crate::audio::decoder::get_audio_metadata;
use serde::Serialize;

#[derive(Serialize)]
pub struct AudioInfo {
    pub duration_s: f32,
    pub size_bytes: u64,
    pub format: String,
    pub bitrate_kbps: Option<u32>,
}

#[tauri::command]
pub async fn get_audio_info(path: String) -> Result<AudioInfo, String> {
    let (duration_s, size_bytes, format) =
        get_audio_metadata(&path).map_err(|e| e.to_string())?;

    let bitrate_kbps = if duration_s > 0.0 {
        Some((size_bytes as f32 * 8.0 / duration_s / 1000.0) as u32)
    } else {
        None
    };

    Ok(AudioInfo { duration_s, size_bytes, format, bitrate_kbps })
}