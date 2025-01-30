// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod partition;
use app::commands::*;
use async_std::task;
use tauri::Manager;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconEvent, TrayIconBuilder, MouseButton, MouseButtonState},
};
use tauri_plugin_fs::FsExt;
async fn p() {
    let mut gs = partition::gs::GlobalStorage::new();
    gs.probe();
    // update global store
    app::global_app::update_global_storage(gs);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    task::spawn(async move { p().await });
    let appp = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let scope = app.fs_scope();
            let _ = scope.allow_file("/tmp/aegis.log");
            let toggle = MenuItem::with_id(app, "toggle", "Toggle", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&toggle])?;
            let _ = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |_app, event| match event.id().as_ref() {
                    "toggle" => {
                        println!("toggle clicked");
                    }
                    _ => (),
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                      button: MouseButton::Left,
                      button_state: MouseButtonState::Up,
                      ..
                    } => {
                      
                      // in this example, let's show and focus the main window when the tray is clicked
                      let app = tray.app_handle();
                      if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                      }
                    }
                    _ => {
                      println!("unhandled event {event:?}");
                    }
                  })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_partitions,
            is_uefi,
            get_timezones,
            install,
            hash_password,
            get_keymaps,
            get_locale,
            get_gs,
            human_to_bytes,
            get_x11_keymaps,
            share_logs,
            get_all_logs,
            save_luks_passphrase
        ])
        
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    // get app handle
    let app_handle = appp.app_handle();
    // update global app handle
    app::global_app::set_global_app_handle(app_handle.clone());

    appp.run(|_app_handle, _event| {});
}
