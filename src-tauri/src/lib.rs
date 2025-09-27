mod commands;

use std::io::Cursor;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use base64::Engine;
use tauri::{menu::{Menu, MenuItem}, tray::TrayIconBuilder, Manager};
use image::{DynamicImage, ImageOutputFormat, RgbaImage};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = commands::HistoryState { items: Mutex::new(Vec::new()) };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::get_history,
            commands::copy_to_clipboard,
            commands::clear_history,
        ])
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let handle = app.handle().clone();
            
            // Tray
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[
                &hide_item,
                &show_item,
                &quit_item,
                ])?;

            TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(1),
                    "hide" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.hide().unwrap();
                    },
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.show().unwrap();
                    },
                    _ => {}
                })
                .build(app)?;

            // Storing history
            std::thread::spawn(move || {
                let mut last_text = String::new();
                let mut last_image: Option<Vec<u8>> = None;

                loop {
                    let clipboard = handle.clipboard();

                    if let Ok(text) = clipboard.read_text() {
                        if text != last_text {
                            let state = handle.state::<commands::HistoryState>();
                            let mut items = state.items.lock().unwrap();
                            items.push(commands::HistoryItem::Text(text.clone()));
                            if items.len() > 50 {
                                items.remove(0);
                            }
                            last_text = text;
                        }
                    }

                    if let Ok(image) = clipboard.read_image() {
                        let bytes = image.rgba().to_vec();

                        if last_image.as_ref() != Some(&bytes) {
                            let dyn_image = RgbaImage::from_raw(
                                image.width() as u32,
                                image.height() as u32,
                                bytes.clone(),
                            )
                            .unwrap();

                            let mut buf = Cursor::new(Vec::new());
                            DynamicImage::ImageRgba8(dyn_image)
                                .write_to(&mut buf, ImageOutputFormat::Png)
                                .unwrap();

                            let base64_png =
                                base64::engine::general_purpose::STANDARD.encode(buf.into_inner());

                            let state = handle.state::<commands::HistoryState>();
                            let mut items = state.items.lock().unwrap();
                            items.push(commands::HistoryItem::Image(base64_png));
                            if items.len() > 50 {
                                items.remove(0);
                            }
                            last_image = Some(bytes);
                        }
                    }

                    thread::sleep(Duration::from_millis(500));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
