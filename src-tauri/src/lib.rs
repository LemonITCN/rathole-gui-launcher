use std::sync::Arc;

use parking_lot::RwLock;
use tauri::{Manager, RunEvent, WindowEvent};

mod commands;
mod error;
mod models;
mod paths;
mod runtime;
mod store;

pub use error::{AppError, AppResult};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            paths::ensure_dirs()?;
            let settings = Arc::new(RwLock::new(store::Settings::load()));
            app.manage::<store::SettingsHandle>(settings);
            let manager = Arc::new(runtime::ProcessManager::new(app.handle().clone()));
            app.manage(manager);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_app_info,
            commands::update_settings,
            commands::list_configs,
            commands::get_server_config,
            commands::get_client_config,
            commands::save_server_config,
            commands::save_client_config,
            commands::delete_config,
            commands::rename_config,
            commands::duplicate_config,
            commands::start_config,
            commands::stop_config,
            commands::list_running,
            commands::get_run_status,
            commands::get_recent_logs,
            commands::check_ports,
            commands::find_external_rathole,
            commands::open_conf_dir,
        ])
        .build(tauri::generate_context!())
        .expect("failed to build tauri application");

    app.run(|handle, event| match event {
        RunEvent::WindowEvent {
            event: WindowEvent::CloseRequested { .. },
            ..
        } => {
            if let Some(mgr) = handle.try_state::<Arc<runtime::ProcessManager>>() {
                let mgr = mgr.inner().clone();
                tauri::async_runtime::block_on(async move {
                    mgr.stop_all().await;
                });
            }
        }
        RunEvent::ExitRequested { .. } => {
            if let Some(mgr) = handle.try_state::<Arc<runtime::ProcessManager>>() {
                let mgr = mgr.inner().clone();
                tauri::async_runtime::block_on(async move {
                    mgr.stop_all().await;
                });
            }
        }
        _ => {}
    });
}
