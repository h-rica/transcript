use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct HardwareInfo {
    pub ram_gb: u32,
    pub cpu_name: String,
    pub tier: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub selected_file: RwSignal<Option<String>>,
    pub selected_model: RwSignal<String>,
    pub hardware_info: RwSignal<Option<HardwareInfo>>,
    pub active_model: RwSignal<String>,
}

pub fn provide_app_state() {
    let state = AppState {
        selected_file: RwSignal::new(None),
        selected_model: RwSignal::new("whisper-tiny".to_string()),
        hardware_info: RwSignal::new(None),
        active_model: RwSignal::new("whisper-tiny".to_string()),
    };
    provide_context(state);
}

pub fn use_app_state() -> AppState {
    use_context::<AppState>().expect("AppState not provided")
}
