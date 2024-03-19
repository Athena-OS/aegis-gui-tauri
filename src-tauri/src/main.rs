mod app;
mod partition;
use std::thread;

use app::commands::*;
use tauri::Manager;

fn main() {
    // probe global storage in the background
    thread::spawn(move || {
        let mut gs = partition::gs::GlobalStorage::new();
        gs.probe();
        // update global store
        app::global_app::update_global_storage(gs);
    });
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
            human_to_bytes
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
