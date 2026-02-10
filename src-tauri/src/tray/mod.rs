use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Emitter, Manager, Runtime,
};

/// Tray icon states matching the app lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayState {
    Idle,
    Listening,
    Processing,
    Error,
}

fn tooltip_for_state(state: TrayState) -> &'static str {
    match state {
        TrayState::Idle => "Dictation — Idle",
        TrayState::Listening => "Dictation — Recording...",
        TrayState::Processing => "Dictation — Processing...",
        TrayState::Error => "Dictation — Error",
    }
}

/// Build and register the system tray icon with its context menu.
pub fn setup<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<TrayIcon<R>> {
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let pause_item = MenuItem::with_id(app, "pause-resume", "Pause", true, None::<&str>)?;
    let about_item = MenuItem::with_id(app, "about", "About Dictation", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[&settings_item, &pause_item, &about_item, &quit_item],
    )?;

    let icon = Image::from_bytes(include_bytes!("../../icons/icon.png"))
        .expect("Failed to load tray icon");

    let tray = TrayIconBuilder::with_id("dictation-tray")
        .icon(icon)
        .tooltip(tooltip_for_state(TrayState::Idle))
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "settings" => {
                    if let Some(win) = app.get_webview_window("settings") {
                        let _ = win.show();
                        let _ = win.set_focus();
                    }
                }
                "pause-resume" => {
                    let _ = app.emit("tray://pause-resume", ());
                }
                "about" => {
                    let version = env!("CARGO_PKG_VERSION");
                    let _ = app.emit("tray://about", version);
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(tray)
}

/// Update the tray tooltip to reflect current app state.
/// TODO: Swap icon per state once distinct icon assets are designed.
pub fn set_state<R: Runtime>(app: &AppHandle<R>, state: TrayState) {
    if let Some(tray) = app.tray_by_id("dictation-tray") {
        let _ = tray.set_tooltip(Some(tooltip_for_state(state)));
    }
}
