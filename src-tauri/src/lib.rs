mod asr;
mod audio;
mod commands;
mod export;
mod models;

use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
};

#[cfg(debug_assertions)]
static DEV_CLEANUP_SCHEDULED: AtomicBool = AtomicBool::new(false);

#[cfg(debug_assertions)]
fn repo_root_from_cwd() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    if cwd
        .file_name()
        .map(|name| name.to_string_lossy() == "src-tauri")
        .unwrap_or(false)
    {
        cwd.parent().map(Path::to_path_buf)
    } else {
        Some(cwd)
    }
}

#[cfg(debug_assertions)]
fn schedule_dev_cleanup() {
    if DEV_CLEANUP_SCHEDULED.swap(true, Ordering::SeqCst) {
        return;
    }

    let Some(repo_root) = repo_root_from_cwd() else {
        return;
    };

    let helper_binary = repo_root
        .join("scripts")
        .join("dev-helper")
        .join("target")
        .join("debug")
        .join(if cfg!(windows) {
            "dev-helper.exe"
        } else {
            "dev-helper"
        });

    if !helper_binary.exists() {
        return;
    }

    let _ = Command::new(helper_binary)
        .arg("cleanup")
        .arg("--app-pid")
        .arg(std::process::id().to_string())
        .arg("--repo-root")
        .arg(repo_root)
        .spawn();
}

#[cfg(not(debug_assertions))]
fn schedule_dev_cleanup() {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(commands::transcribe::TranscriptionControl::default())
        .on_window_event(|window, event| {
            if window.label() == "main"
                && matches!(event, tauri::WindowEvent::CloseRequested { .. })
            {
                schedule_dev_cleanup();
            }
        })
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
