// global App is a global variable that stores the app for sending events
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::{Manager, AppHandle}; 

pub static GLOBAL_APP_HANDLE: Lazy<Mutex<Option<AppHandle>>> = Lazy::new(|| Mutex::new(None));

pub fn set_global_app_handle(app_handle: AppHandle) {
    let mut handle_lock = GLOBAL_APP_HANDLE.lock().unwrap(); // Get a lock
    *handle_lock = Some(app_handle); // Set the value
}

pub fn emit_global_event(event: &str, payload:&str) {
    let handle_lock = GLOBAL_APP_HANDLE.lock().unwrap(); // Lock the Mutex
    if let Some(app_handle) = handle_lock.as_ref() {
        app_handle.emit_all(event, payload).expect("Failed to emit log event");
    } else {
        // Handle the case where AppHandle has not been set. A rare scenario.
        println!("Global AppHandle has not been initialized.");
    }
}

