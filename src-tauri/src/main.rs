mod app;
mod partition;
use app::commands::{
    get_gs, get_keymaps, get_locale, get_partitions, get_timezones, hash_password, human_to_bytes,
    is_uefi, save_conf,
};
use tauri::Manager;

fn main() {
    let appp = tauri::Builder::default()
        
        .invoke_handler(tauri::generate_handler![
            get_partitions,
            is_uefi,
            get_timezones,
            save_conf,
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
    app::logger::Logger::start(app_handle);
    // run the app
    appp.run(|_app_handle, _event| {});
}
