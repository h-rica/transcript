use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct HardwareInfo {
    pub ram_gb: u32,
    pub cpu_name: String,
    pub gpu_vram_gb: Option<u32>,
    pub tier: String,
}

fn tier_for_ram(ram_gb: u32) -> &'static str {
    if ram_gb >= 24 {
        "heavy"
    } else if ram_gb >= 12 {
        "balanced"
    } else {
        "minimal"
    }
}

#[tauri::command]
pub async fn get_hardware_info() -> Result<HardwareInfo, String> {
    let system = System::new_all();
    let ram_gb = ((system.total_memory() as f64) / 1024_f64.powi(3)).ceil() as u32;
    let cpu_name = system
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .filter(|brand| !brand.trim().is_empty())
        .unwrap_or_else(|| "Unknown CPU".into());

    Ok(HardwareInfo {
        ram_gb: ram_gb.max(1),
        cpu_name,
        gpu_vram_gb: None,
        tier: tier_for_ram(ram_gb.max(1)).into(),
    })
}

#[cfg(test)]
mod tests {
    use super::tier_for_ram;

    #[test]
    fn ram_tier_thresholds_are_stable() {
        assert_eq!(tier_for_ram(8), "minimal");
        assert_eq!(tier_for_ram(16), "balanced");
        assert_eq!(tier_for_ram(32), "heavy");
    }
}
