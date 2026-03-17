mod asr;
mod audio;
mod commands;
mod export;
mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::transcribe::transcribe_file,
            commands::transcribe::cancel_transcription,
            commands::audio::get_audio_info,
            commands::hardware::get_hardware_info,
            commands::models::get_models,
            commands::models::download_model,
            commands::models::delete_model,
            commands::export::export_transcript,
            commands::settings::get_settings,
            commands::settings::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
