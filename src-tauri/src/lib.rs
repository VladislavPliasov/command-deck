mod commands;
mod discovery;
mod registry;

use registry::storage::Storage;
use registry::RegistryService;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir()?.join("registry");
            let storage = Storage::new(dir)?;
            app.manage(RegistryService::new(storage));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_info::app_info,
            commands::registry::registry_list,
            commands::registry::registry_get,
            commands::registry::registry_register,
            commands::registry::registry_update,
            commands::registry::registry_remove,
            commands::discovery::discovery_resolve,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
