use base64::Engine;
use tauri::{AppHandle, image::Image};
use tauri_plugin_clipboard_manager::ClipboardExt;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum HistoryItem {
    Text(String),
    Image(String),
}

pub struct HistoryState {
    pub items: Mutex<Vec<HistoryItem>>,
}

#[tauri::command]
pub fn get_history(state: tauri::State<HistoryState>) -> Vec<HistoryItem> {
    state.items.lock().unwrap().clone()
}

#[tauri::command]
pub fn copy_to_clipboard(app: AppHandle, item: HistoryItem) -> Result<(), String> {
    let clipboard = app.clipboard();

    match item {
        HistoryItem::Text(text) => {
            clipboard.write_text(text).map_err(|e| e.to_string())?;
        }
        HistoryItem::Image(base64_png) => {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(base64_png)
                .map_err(|e| e.to_string())?;

            let dyn_img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
            let rgba_img = dyn_img.into_rgba8();
            let width = rgba_img.width();
            let height = rgba_img.height();
            let data = rgba_img.into_raw();

            let tauri_img = Image::new_owned(data, width, height);
            clipboard.write_image(&tauri_img).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn clear_history(state: tauri::State<HistoryState>) {
    state.items.lock().unwrap().clear();
}
