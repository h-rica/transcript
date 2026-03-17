use crate::state::app_state::{ExportRequest, UiError};

pub async fn export_transcript(request: ExportRequest) -> Result<(), UiError> {
    if tauri_sys::core::is_tauri() {
        tauri_sys::core::invoke_result::<(), String>("export_transcript", &request)
            .await
            .map_err(UiError::from)
    } else {
        Ok(())
    }
}
