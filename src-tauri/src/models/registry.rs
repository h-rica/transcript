use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct RegistryFile {
    models: Vec<RegistryModel>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RegistryModel {
    pub id: String,
    pub name: String,
    pub size_mb: u32,
    pub tier: String,
    pub bundled: bool,
    pub diarization: bool,
    pub languages: Vec<String>,
    pub source: String,
    #[serde(rename = "repo_id")]
    pub _repo_id: Option<String>,
    pub files: Option<Vec<String>>,
    #[serde(rename = "sha256")]
    pub _sha256: Option<HashMap<String, String>>,
}

fn repo_root() -> PathBuf {
    let cwd = std::env::current_dir().expect("current dir");
    if cwd
        .file_name()
        .map(|name| name == "src-tauri")
        .unwrap_or(false)
    {
        cwd.parent().map(PathBuf::from).unwrap_or(cwd)
    } else {
        cwd
    }
}

pub fn registry_path() -> PathBuf {
    repo_root().join("models").join("registry.toml")
}

pub fn resources_root() -> PathBuf {
    repo_root().join("src-tauri").join("resources")
}

pub fn load_registry() -> Result<Vec<RegistryModel>> {
    let path = registry_path();
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("failed to read model registry at {}", path.display()))?;
    let file: RegistryFile = toml::from_str(&contents)
        .with_context(|| format!("failed to parse model registry at {}", path.display()))?;
    Ok(file.models)
}

pub fn model_is_installed(model: &RegistryModel) -> bool {
    if model.bundled {
        return true;
    }

    let Some(files) = &model.files else {
        return false;
    };
    let root = resources_root();
    files.iter().all(|file| root.join(file).exists())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_expected_models() {
        let models = load_registry().expect("registry should parse");
        assert!(models.iter().any(|model| model.id == "whisper-tiny"));
        assert!(models.iter().any(|model| model.id == "vibevoice-int8"));
    }
}
