use serde::Serialize;

#[derive(Serialize)]
pub struct HardwareInfo {
    pub ram_gb: u32,
    pub cpu_name: String,
    pub gpu_vram_gb: Option<u32>,
    pub tier: String,
}

#[tauri::command]
pub async fn get_hardware_info() -> Result<HardwareInfo, String> {
    Err("Not implemented".into())
}
