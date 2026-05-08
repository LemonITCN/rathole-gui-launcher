use std::sync::Arc;

use parking_lot::RwLock;
use tauri::{Manager, RunEvent, WindowEvent};

mod commands;
mod error;
mod models;
mod paths;
mod runtime;
mod store;
mod tray;

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
            let manager_for_adopt = manager.clone();
            app.manage(manager);
            tray::setup(app.handle())?;

            // Re-attach to any rathole process the previous launcher session
            // left running. If the launcher was killed unexpectedly the
            // child rathole stays alive (re-parented to launchd / init);
            // adopting it lets the user manage it again instead of having
            // an invisible orphan.
            tauri::async_runtime::spawn(async move {
                let state = store::runtime_state::load();
                for inst in state.instances {
                    let Ok(mode) = paths::Mode::from_str(&inst.mode) else {
                        store::runtime_state::remove(&inst.mode, &inst.name);
                        continue;
                    };
                    if manager_for_adopt
                        .clone()
                        .adopt(mode, inst.name.clone(), inst.pid, inst.started_at)
                        .await
                        .is_err()
                    {
                        store::runtime_state::remove(&inst.mode, &inst.name);
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            // Closing the main window only hides it; the app keeps running in
            // the menu bar / tray with the rathole instances still alive.
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
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
            commands::parse_server_toml,
            commands::check_rathole_update,
            commands::download_rathole_release,
        ])
        .build(tauri::generate_context!())
        .expect("failed to build tauri application");

    app.run(|handle, event| match event {
        RunEvent::ExitRequested { .. } => {
            // Real quit (Cmd+Q, tray Quit, or programmatic exit). Stop every
            // rathole child before letting the process leave so the user
            // never finds an orphaned daemon after closing the launcher.
            if let Some(mgr) = handle.try_state::<Arc<runtime::ProcessManager>>() {
                let mgr = mgr.inner().clone();
                tauri::async_runtime::block_on(async move {
                    mgr.stop_all().await;
                });
            }
        }
        // `Reopen` is a macOS-only variant (Dock click while the window is
        // hidden); it does not exist in `RunEvent` on Windows / Linux.
        #[cfg(target_os = "macos")]
        RunEvent::Reopen {
            has_visible_windows,
            ..
        } => {
            if !has_visible_windows {
                if let Some(window) = handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
        _ => {}
    });
}
