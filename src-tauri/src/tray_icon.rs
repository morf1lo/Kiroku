use tauri::{menu::{Menu, MenuItem}, tray::{TrayIconBuilder, TrayIconEvent}, AppHandle, Manager};

pub fn build(handle: &AppHandle) -> tauri::Result<()> {
    let quit_item = MenuItem::with_id(handle, "quit", "Quit", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(handle, "hide", "Hide", true, None::<&str>)?;
    let show_item = MenuItem::with_id(handle, "show", "Show", true, None::<&str>)?;
    let menu = Menu::with_items(handle, &[
        &show_item,
        &hide_item,
        &quit_item,
    ])?;

    let tray_handle = handle.clone();
    TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Kiroku")
        .show_menu_on_left_click(false)
        .icon(handle.default_window_icon().unwrap().clone())
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
        .build(handle)?;

    Ok(())
}
