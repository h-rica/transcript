use crate::state::app_state::HardwareInfo;

#[derive(Clone, Copy)]
pub struct UiModel {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub tier: &'static str,
    pub diarization: bool,
    pub ready: bool,
    pub rtfx: f32,
    pub size_mb: u32,
}

pub const AVAILABLE_MODELS: [UiModel; 3] = [
    UiModel {
        id: "whisper-tiny",
        name: "Whisper Tiny",
        description: "Bundled fallback for Phase 1. Fastest path to the first local transcript.",
        tier: "entry",
        diarization: false,
        ready: true,
        rtfx: 1.65,
        size_mb: 75,
    },
    UiModel {
        id: "whisper-medium",
        name: "Whisper Medium",
        description: "Higher accuracy with a larger local footprint. Visible before download wiring is complete.",
        tier: "balanced",
        diarization: false,
        ready: false,
        rtfx: 0.72,
        size_mb: 1420,
    },
    UiModel {
        id: "whisper-large-v3",
        name: "Whisper Large v3",
        description: "Best quality and speaker support, but too heavy for the current Phase 1 bundle.",
        tier: "heavy",
        diarization: true,
        ready: false,
        rtfx: 0.45,
        size_mb: 3000,
    },
];

pub fn selected_model(model_id: &str) -> UiModel {
    AVAILABLE_MODELS
        .iter()
        .find(|model| model.id == model_id)
        .copied()
        .unwrap_or(AVAILABLE_MODELS[0])
}

pub fn hardware_warning(hardware: Option<HardwareInfo>, model_tier: &str) -> Option<String> {
    let hardware = hardware?;
    if model_tier == "heavy" && hardware.ram_gb < 16 {
        Some(format!(
            "{} GB RAM on {} is unlikely to handle the heavy profile comfortably in Phase 1.",
            hardware.ram_gb, hardware.cpu_name
        ))
    } else {
        None
    }
}

pub fn format_hms(seconds: f32) -> String {
    let total = seconds.max(0.0).round() as u32;
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let secs = total % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{secs:02}")
    } else {
        format!("{minutes:02}:{secs:02}")
    }
}

pub fn format_mm_ss(seconds: f32) -> String {
    let total = seconds.max(0.0).round() as u32;
    let minutes = total / 60;
    let secs = total % 60;
    format!("{minutes:02}:{secs:02}")
}

pub fn format_elapsed(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    if hours > 0 {
        format!("{hours}:{minutes:02}:{secs:02}")
    } else {
        format!("{minutes:02}:{secs:02}")
    }
}

pub fn format_bytes(size: u64) -> String {
    let kb = 1024.0;
    let mb = kb * 1024.0;
    let gb = mb * 1024.0;
    let size = size as f64;

    if size >= gb {
        format!("{:.2} GB", size / gb)
    } else if size >= mb {
        format!("{:.1} MB", size / mb)
    } else if size >= kb {
        format!("{:.1} KB", size / kb)
    } else {
        format!("{size:.0} B")
    }
}

pub fn status_label(status: &crate::state::app_state::TranscriptionStatus) -> String {
    match status {
        crate::state::app_state::TranscriptionStatus::Idle => "Waiting to start".into(),
        crate::state::app_state::TranscriptionStatus::LoadingModel => "Loading model".into(),
        crate::state::app_state::TranscriptionStatus::Running => "Transcribing".into(),
        crate::state::app_state::TranscriptionStatus::Complete => "Complete".into(),
        crate::state::app_state::TranscriptionStatus::Failed(message) => {
            format!("Failed: {message}")
        }
    }
}

pub fn speaker_initial(speaker: &str) -> String {
    speaker
        .chars()
        .find(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_uppercase().to_string())
        .unwrap_or_else(|| "S".into())
}

pub fn speaker_palette(speaker: &str) -> (&'static str, &'static str) {
    let hash = speaker
        .bytes()
        .fold(0u8, |acc, byte| acc.wrapping_add(byte))
        % 4;
    match hash {
        0 => ("#dbeafe", "#1d4ed8"),
        1 => ("#fce7f3", "#9d174d"),
        2 => ("#dcfce7", "#166534"),
        _ => ("#ede9fe", "#5b21b6"),
    }
}
