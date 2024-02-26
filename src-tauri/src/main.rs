mod app;
mod partition;
use app::commands::{
    get_partitions,
    is_uefi,
    get_timezones,
    hash_password,
    get_keymaps,
    get_locale,
    save_conf,
    get_gs,
};
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_partitions,
            is_uefi,
            get_timezones,
            save_conf,
            hash_password,
            get_keymaps,
            get_locale,
            get_gs,
        ])
        .plugin(tauri_plugin_system_info::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");   
}
