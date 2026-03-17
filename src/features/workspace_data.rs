use serde::{Deserialize, Serialize};

use crate::state::app_state::{HardwareInfo, Settings, UiError, WorkspaceModel};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RemoteModelRecord {
    id: String,
    name: String,
    size_mb: u32,
    tier: String,
    bundled: bool,
    diarization: bool,
    languages: Vec<String>,
    source: String,
    status: String,
}

#[derive(Clone, Copy)]
struct ModelProfile {
    id: &'static str,
    description: &'static str,
    rtfx: f32,
}

const MODEL_PROFILES: [ModelProfile; 4] = [
    ModelProfile {
        id: "whisper-tiny",
        description: "Fast bundled fallback for short local transcriptions.",
        rtfx: 1.65,
    },
    ModelProfile {
        id: "whisper-medium",
        description: "Balanced accuracy profile for longer recordings and multilingual speech.",
        rtfx: 0.72,
    },
    ModelProfile {
        id: "whisper-large-v3",
        description: "High-accuracy multilingual model with the heaviest local footprint.",
        rtfx: 0.45,
    },
    ModelProfile {
        id: "vibevoice-int8",
        description: "Speaker-aware INT8 pipeline tuned for French and English diarization.",
        rtfx: 1.30,
    },
];

fn profile_for(model_id: &str) -> ModelProfile {
    MODEL_PROFILES
        .iter()
        .find(|profile| profile.id == model_id)
        .copied()
        .unwrap_or(ModelProfile {
            id: "unknown",
            description: "Local transcription model.",
            rtfx: 1.0,
        })
}

fn map_model(record: RemoteModelRecord) -> WorkspaceModel {
    let profile = profile_for(&record.id);
    WorkspaceModel {
        id: record.id,
        name: record.name,
        description: profile.description.into(),
        size_mb: record.size_mb,
        tier: record.tier,
        bundled: record.bundled,
        diarization: record.diarization,
        languages: record.languages,
        source: record.source,
        status: record.status,
        rtfx: profile.rtfx,
    }
}

pub fn fallback_models() -> Vec<WorkspaceModel> {
    [
        RemoteModelRecord {
            id: "whisper-tiny".into(),
            name: "Whisper Tiny".into(),
            size_mb: 150,
            tier: "minimal".into(),
            bundled: true,
            diarization: false,
            languages: vec!["fr".into(), "en".into(), "multilingual".into()],
            source: "bundled".into(),
            status: "bundled".into(),
        },
        RemoteModelRecord {
            id: "whisper-medium".into(),
            name: "Whisper Medium".into(),
            size_mb: 1500,
            tier: "standard".into(),
            bundled: false,
            diarization: false,
            languages: vec!["fr".into(), "en".into(), "multilingual".into()],
            source: "huggingface".into(),
            status: "missing".into(),
        },
        RemoteModelRecord {
            id: "whisper-large-v3".into(),
            name: "Whisper Large v3".into(),
            size_mb: 3100,
            tier: "standard".into(),
            bundled: false,
            diarization: false,
            languages: vec!["fr".into(), "en".into(), "multilingual".into()],
            source: "huggingface".into(),
            status: "missing".into(),
        },
        RemoteModelRecord {
            id: "vibevoice-int8".into(),
            name: "VibeVoice INT8".into(),
            size_mb: 8500,
            tier: "standard".into(),
            bundled: false,
            diarization: true,
            languages: vec!["fr".into(), "en".into()],
            source: "huggingface".into(),
            status: "missing".into(),
        },
    ]
    .into_iter()
    .map(map_model)
    .collect()
}

pub fn selected_model(models: &[WorkspaceModel], selected_id: &str) -> WorkspaceModel {
    models
        .iter()
        .find(|model| model.id == selected_id)
        .cloned()
        .or_else(|| {
            fallback_models()
                .into_iter()
                .find(|model| model.id == selected_id)
        })
        .unwrap_or_else(|| {
            fallback_models()
                .into_iter()
                .next()
                .expect("fallback models")
        })
}

pub fn model_is_ready(model: &WorkspaceModel) -> bool {
    matches!(model.status.as_str(), "bundled" | "downloaded")
}

pub fn installed_storage_mb(models: &[WorkspaceModel]) -> u32 {
    models
        .iter()
        .filter(|model| model_is_ready(model))
        .map(|model| model.size_mb)
        .sum()
}

pub fn storage_capacity_mb(models: &[WorkspaceModel], hardware: Option<HardwareInfo>) -> u32 {
    let ram_gb = hardware.map(|info| info.ram_gb).unwrap_or(16);
    let base = ram_gb.saturating_mul(1024).max(12_288);
    base.max(installed_storage_mb(models).saturating_add(2_048))
}

pub fn recommended_model_id(
    models: &[WorkspaceModel],
    hardware: Option<HardwareInfo>,
) -> Option<String> {
    let ram_gb = hardware.map(|info| info.ram_gb).unwrap_or(8);
    if ram_gb >= 24 {
        models
            .iter()
            .find(|model| model.id == "vibevoice-int8")
            .map(|model| model.id.clone())
            .or_else(|| models.first().map(|model| model.id.clone()))
    } else if ram_gb >= 12 {
        models
            .iter()
            .find(|model| model.id == "whisper-medium")
            .map(|model| model.id.clone())
            .or_else(|| models.first().map(|model| model.id.clone()))
    } else {
        models.first().map(|model| model.id.clone())
    }
}

pub async fn load_workspace_models() -> Result<Vec<WorkspaceModel>, UiError> {
    if tauri_sys::core::is_tauri() {
        tauri_sys::core::invoke_result::<Vec<RemoteModelRecord>, String>("get_models", &())
            .await
            .map(|records| records.into_iter().map(map_model).collect())
            .map_err(UiError::from)
    } else {
        Ok(fallback_models())
    }
}

pub async fn load_settings() -> Result<Settings, UiError> {
    if tauri_sys::core::is_tauri() {
        tauri_sys::core::invoke_result::<Settings, String>("get_settings", &())
            .await
            .map_err(UiError::from)
    } else {
        Ok(Settings::default())
    }
}

pub async fn save_settings(settings: Settings) -> Result<(), UiError> {
    if tauri_sys::core::is_tauri() {
        tauri_sys::core::invoke_result::<(), String>("save_settings", &settings)
            .await
            .map_err(UiError::from)
    } else {
        Ok(())
    }
}

pub async fn load_hardware_info() -> Result<HardwareInfo, UiError> {
    if tauri_sys::core::is_tauri() {
        tauri_sys::core::invoke_result::<HardwareInfo, String>("get_hardware_info", &())
            .await
            .map_err(UiError::from)
    } else {
        Ok(HardwareInfo {
            ram_gb: 16,
            cpu_name: "Offline preview CPU".into(),
            gpu_vram_gb: None,
            tier: "balanced".into(),
        })
    }
}
