// global App is a global variable that stores the app for sending events
use crate::{app, partition};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
pub static GLOBAL_APP_HANDLE: Lazy<Mutex<Option<AppHandle>>> = Lazy::new(|| Mutex::new(None));

pub fn set_global_app_handle(app_handle: AppHandle) {
    match GLOBAL_APP_HANDLE.lock() {
        Ok(mut handle_lock) => {
            *handle_lock = Some(app_handle);
        }
        Err(_) => {},
    }
}

pub fn emit_global_event(event: &str, payload: &str) {
    let handle_lock_result = GLOBAL_APP_HANDLE.lock();

    match handle_lock_result {
        Ok(handle_lock) => {
            if let Some(app_handle) = handle_lock.as_ref() {
                // Attempt to emit the event, handling any potential error
                let _ = app_handle.emit(event, payload);
            }
        }
        Err(e) => {
            println!("Failed to acquire lock on GLOBAL_APP_HANDLE: {}", e);
        }
    }
}

// total install items
static TOTAL_ITEMS: i32 = 28;
// This struct stores information such as the install percentage,
// The global store.
// The config etc.
pub struct GlobalStore {
    // Install percentage
    progress: i32,
    // config. This way, we can access config anywhere during installation
    config: app::config::Config,
    // globalStorage. This is important so that we dont have to probe gs
    // each time we need to use it during installation
    gs: partition::gs::GlobalStorage,
}

impl GlobalStore {
    pub fn new() -> Self {
        GlobalStore {
            progress: 0,
            config: app::config::Config::default(),
            gs: partition::gs::GlobalStorage::new(),
        }
    }
}
pub static GLOBAL_STORE: Lazy<Mutex<GlobalStore>> = Lazy::new(|| Mutex::new(GlobalStore::new()));

// increases the progress by one and emit the progress event
pub fn update_progress() {
    match GLOBAL_STORE.lock() {
        Ok(mut global_store) => {
            global_store.progress += 1;
            let progress = ((global_store.progress + 1) as f64 / TOTAL_ITEMS as f64) * 100.0;
            emit_global_event("percentage", &progress.to_string())
        }
        Err(_) => {},
    }
}

// update the config in global store
pub fn update_config(config: app::config::Config) {
    match GLOBAL_STORE.lock() {
        Ok(mut global_store) => {
            global_store.config = config;
        }
        Err(_e) => {}
    }
}

// update global store
pub fn update_global_storage(gs: partition::gs::GlobalStorage) {
    match GLOBAL_STORE.lock() {
        Ok(mut global_store) => {
            global_store.gs = gs;
        }
        Err(_) => {},
    }
}

pub fn get_global_storage() -> Result<partition::gs::GlobalStorage, Box<dyn std::error::Error>> {
    match GLOBAL_STORE.lock() {
        Ok(global_store) => Ok(global_store.gs.clone()),
        Err(e) => {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock of global store. error:{}", e),
            )))
        }
    }
}

pub fn get_config() -> Result<app::config::Config, Box<dyn std::error::Error>> {
    match GLOBAL_STORE.lock() {
        Ok(global_store) => Ok(global_store.config.clone()),
        Err(e) => {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire lock of global store. error:{}", e),
            )))
        }
    }
}
