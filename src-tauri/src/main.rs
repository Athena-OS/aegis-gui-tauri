mod app;
mod partition;
use app::commands::{
    get_gs, get_keymaps, get_locale, get_partitions, get_timezones, hash_password, human_to_bytes,
    is_uefi, save_conf,
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
            human_to_bytes
        ])
        .plugin(tauri_plugin_system_info::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
