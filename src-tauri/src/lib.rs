mod commands;

use std::hash::{DefaultHasher, Hash, Hasher};
use std::{collections::VecDeque, io::Cursor};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use base64::Engine;
use tauri::{menu::{Menu, MenuItem}, tray::{TrayIconBuilder, TrayIconEvent}, Manager, WindowEvent};
use image::{DynamicImage, ImageOutputFormat, RgbaImage};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_history,
            commands::copy_to_clipboard,
            commands::clear_history,
        ])
        .on_window_event(move |window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide();
            },
            _ => {}
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let path = app
                .path()
                .resolve("history.json", tauri::path::BaseDirectory::AppConfig)?;

            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let state = commands::HistoryState {
                items: Mutex::new(VecDeque::new()),
                last_text: Mutex::new(String::new()),
                last_image: Mutex::new(String::new()),
                last_image_hash: Mutex::new(0u64),
                file_path: path,
            };

            state.load();

            app.manage(state);

            let handle = app.handle().clone();
            
            // Tray
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[
                &show_item,
                &hide_item,
                &quit_item,
            ])?;

            let tray_handle = handle.clone();
            TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Kiroku")
                .show_menu_on_left_click(false)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => app.exit(1),
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        let _ = window.show();
                    },
                    "hide" => {
                        let window = app.get_webview_window("main").unwrap();
                        let _ = window.hide();
                    },
                    _ => {}
                })
                .on_tray_icon_event(move |_tray, event| match event {
                    TrayIconEvent::DoubleClick {
                        id: _,
                        position: _,
                        rect: _,
                        button: _,
                    } => {
                        if let Some(window) = tray_handle.get_webview_window("main") {
                            let _ = window.show();
                        }
                    },
                    _ => {}
                })
                .build(app)?;

            // Storing history
            let thread_handle = handle.clone();
            std::thread::spawn(move || {
                loop {
                    let clipboard = thread_handle.clipboard();
                    let state = thread_handle.state::<commands::HistoryState>();

                    if let Ok(text) = clipboard.read_text() {
                        let mut last_text = state.last_text.lock().unwrap();
                        if text != *last_text {
                            let mut items = state.items.lock().unwrap();
                            items.push_back(commands::HistoryItem::Text(text.clone()));
                            if items.len() > 50 {
                                items.pop_front();
                            }
                            *last_text = text;
                        }
                        
                        state.save_to_file();
                    }

                    if let Ok(image) = clipboard.read_image() {
                        let bytes = image.rgba().to_vec();

                        let hash = hash_bytes(&bytes);

                        let mut last_hash = state.last_image_hash.lock().unwrap();
                        if *last_hash != hash {
                            let dyn_image = RgbaImage::from_raw(
                                image.width() as u32,
                                image.height() as u32,
                                bytes.clone(),
                            ).unwrap();

                            let mut buf = Cursor::new(Vec::new());
                            DynamicImage::ImageRgba8(dyn_image)
                                .write_to(&mut buf, ImageOutputFormat::Png)
                                .unwrap();

                            let base64_png =
                                base64::engine::general_purpose::STANDARD.encode(buf.into_inner());

                            let mut items = state.items.lock().unwrap();
                            items.push_back(commands::HistoryItem::Image(base64_png.clone()));
                            if items.len() > 50 {
                                items.pop_front();
                            }

                            *last_hash = hash;
                        }

                        state.save_to_file();
                    }

                    thread::sleep(Duration::from_millis(2000));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}
