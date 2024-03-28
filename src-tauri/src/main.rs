// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod partition;
use async_std::task;
use app::commands::*;
use tauri::Manager;

async fn p() {
    let mut gs = partition::gs::GlobalStorage::new();
    gs.probe();
    // update global store
    app::global_app::update_global_storage(gs);
}
fn main() {
    // probe global storage in the background
    task::spawn(async move {p().await});
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
            app::logger::share_logs,
            app::logger::get_all_logs
        ])
        .plugin(tauri_plugin_system_info::init())
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
