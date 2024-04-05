// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod partition;
use app::commands::*;
use async_std::task;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

async fn p() {
    let mut gs = partition::gs::GlobalStorage::new();
    gs.probe();
    // update global store
    app::global_app::update_global_storage(gs);
}
fn main() {
    // probe global storage in the background
    task::spawn(async move { p().await });

    // System tray only introduced if the user closes by mistake
    //let tray_menu = SystemTrayMenu::new(); // insert the menu items here
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(show);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    let appp = tauri::Builder::default()
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
            app::logger::share_logs,
            app::logger::get_all_logs,
            save_luks_passphrase
        ])
        .plugin(tauri_plugin_system_info::init())
        .system_tray(system_tray)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    // get app handle
    let app_handle = appp.app_handle();
    // update global app handle
    app::global_app::set_global_app_handle(app_handle.clone());
    // start global log collector
    //app::logger::Logger::start(app_handle.clone());
    app::logger::setup_logging(app_handle.clone());
    // run the app

    appp.run(|_app_handle, _event| {});
}
