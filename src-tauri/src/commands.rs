use base64::Engine;
use tauri::{image::Image, AppHandle};
use tauri_plugin_clipboard_manager::ClipboardExt;
use std::{collections::VecDeque, path::PathBuf, sync::Mutex};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum HistoryItem {
    Text(String),
    Image(String),
}

pub struct HistoryState {
    pub items: Mutex<VecDeque<HistoryItem>>,
    pub last_text: Mutex<String>,
    pub last_image: Mutex<String>,
    pub last_image_hash: Mutex<u64>,
    pub file_path: PathBuf,
}

impl HistoryState {
    pub fn save_to_file(&self) {
        if let Ok(items) = self.items.lock() {
            let json = serde_json::to_string(&*items).unwrap_or_default();
            let _ = std::fs::write(&self.file_path, json);
        }
    }

    pub fn load(&self) {
        if let Ok(data) = std::fs::read_to_string(&self.file_path) {
            if let Ok(vec) = serde_json::from_str::<VecDeque<HistoryItem>>(&data) {
                if let Ok(mut items) = self.items.lock() {
                    *items = vec;
                }
            }
        }
    }
}

#[tauri::command]
pub fn get_history(state: tauri::State<HistoryState>) -> VecDeque<HistoryItem> {
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
pub fn clear_history(state: tauri::State<HistoryState>, app: AppHandle) {
    state.items.lock().unwrap().clear();
    state.last_text.lock().unwrap().clear();
    state.last_image.lock().unwrap().clear();
    *state.last_image_hash.lock().unwrap() = 0u64;

    let clipboard = app.clipboard();
    let _ = clipboard.write_text("");

    state.save_to_file();
}
