//! App wiring: load catalog, register the global shortcut, expose commands.

mod commands;

use std::sync::Mutex;

use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{
    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
};

use cheatkeys_core::catalog::Catalog;
use commands::AppState;

/// The default launcher shortcut: Ctrl+Shift+Period.
fn launcher_shortcut() -> Shortcut {
    Shortcut::new(
        Some(Modifiers::CONTROL | Modifiers::SHIFT),
        Code::Period,
    )
}

/// Toggle the main window's visibility + focus. Runs when the shortcut fires.
fn toggle_launcher(app: &tauri::AppHandle) {
    let Some(window) = app.get_webview_window("main") else {
        tracing::warn!("main window missing on shortcut");
        return;
    };
    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        _ => {
            let _ = window.show();
            let _ = window.set_focus();
            // Let the frontend reset the overlay to a fresh search.
            let _ = app.emit("launcher-opened", ());
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let shortcut = launcher_shortcut();

    let result = tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, scut, event| {
                    if scut == &shortcut && event.state() == ShortcutState::Pressed {
                        toggle_launcher(app);
                    }
                })
                .build(),
        )
        .setup(move |app| {
            // Resolve the bundled cheat sheets directory and load the catalog.
            let dir = app
                .path()
                .resolve("cheatsheets", tauri::path::BaseDirectory::Resource)
                .unwrap_or_else(|_| std::path::PathBuf::from("cheatsheets"));

            let catalog = Catalog::load_from_dir(&dir).unwrap_or_default();
            app.manage(AppState {
                catalog: Mutex::new(catalog),
            });

            // Register the global shortcut now that the app is set up.
            app.global_shortcut().register(shortcut)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_cheat_sheets,
            commands::get_cheat_sheet
        ])
        .run(tauri::generate_context!());

    // Handle a failed startup explicitly instead of panicking (security
    // steering rule 8): log and exit with a non-zero code.
    if let Err(e) = result {
        tracing::error!("failed to start CheatKeys: {e}");
        std::process::exit(1);
    }
}
