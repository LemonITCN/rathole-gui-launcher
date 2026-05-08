use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;
use tauri::menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, Wry};

use crate::paths::Mode;
use crate::runtime::{InstanceState, ProcessManager};
use crate::store::{self, SettingsHandle};

const TRAY_ID: &str = "main-tray";

pub struct TrayHolder {
    icon: TrayIcon<Wry>,
}

pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    let menu = build_menu(app)?;

    let icon_image = app
        .default_window_icon()
        .cloned()
        .expect("default window icon must be configured in tauri.conf.json");

    let tray = TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon_image)
        // The launcher's icon is full-color, so we tell macOS *not* to treat
        // it as a template. Template mode collapses any non-transparent pixel
        // into a single foreground colour, which is what turned the icon
        // into a white blob on dark menu bars.
        .icon_as_template(false)
        .tooltip("Rathole Launcher")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(handle_menu_event)
        .on_tray_icon_event(handle_tray_icon_event)
        .build(app)?;

    app.manage(TrayHolder { icon: tray });
    Ok(())
}

pub fn rebuild_menu(app: &AppHandle) {
    let menu = match build_menu(app) {
        Ok(m) => m,
        Err(_) => return,
    };
    if let Some(holder) = app.try_state::<TrayHolder>() {
        let _ = holder.icon.set_menu(Some(menu));
    }
}

#[derive(Debug)]
struct ConfigEntry {
    mode: Mode,
    name: String,
    state: Option<InstanceState>, // None means: not in ProcessManager (cold)
}

fn build_menu(app: &AppHandle) -> tauri::Result<Menu<Wry>> {
    let lang = app
        .try_state::<SettingsHandle>()
        .map(|s| s.read().language.clone().unwrap_or_default())
        .unwrap_or_default();

    let entries = collect_entries(app);
    let running_count = entries
        .iter()
        .filter(|e| matches!(e.state, Some(InstanceState::Running) | Some(InstanceState::Starting)))
        .count();

    let mut items: Vec<Box<dyn IsMenuItem<Wry>>> = Vec::new();

    if entries.is_empty() {
        items.push(Box::new(MenuItem::with_id(
            app,
            "header-empty",
            t(&lang, "no-configs"),
            false,
            None::<&str>,
        )?));
    } else {
        items.push(Box::new(MenuItem::with_id(
            app,
            "header-summary",
            format!(
                "{} {}/{}",
                t(&lang, "running-prefix"),
                running_count,
                entries.len()
            ),
            false,
            None::<&str>,
        )?));

        for entry in &entries {
            let submenu = build_config_submenu(app, &lang, entry)?;
            items.push(Box::new(submenu));
        }
    }

    items.push(Box::new(PredefinedMenuItem::separator(app)?));
    items.push(Box::new(MenuItem::with_id(
        app,
        "show-window",
        t(&lang, "show"),
        true,
        None::<&str>,
    )?));
    items.push(Box::new(PredefinedMenuItem::separator(app)?));
    items.push(Box::new(MenuItem::with_id(
        app,
        "quit-app",
        t(&lang, "quit"),
        true,
        None::<&str>,
    )?));

    let refs: Vec<&dyn IsMenuItem<Wry>> = items.iter().map(|b| b.as_ref()).collect();
    Menu::with_items(app, &refs)
}

fn build_config_submenu(
    app: &AppHandle,
    lang: &str,
    entry: &ConfigEntry,
) -> tauri::Result<Submenu<Wry>> {
    let mode_str = entry.mode.as_str();
    let state_label = match entry.state {
        Some(InstanceState::Running) => t(lang, "state-running"),
        Some(InstanceState::Starting) => t(lang, "state-starting"),
        Some(InstanceState::Stopping) => t(lang, "state-stopping"),
        Some(InstanceState::Exited) | None => t(lang, "state-stopped"),
    };
    let bullet = match entry.state {
        Some(InstanceState::Running) | Some(InstanceState::Starting) => "●",
        Some(InstanceState::Stopping) => "◐",
        _ => "○",
    };
    let title = format!(
        "{} [{}] {} · {}",
        bullet,
        mode_label(lang, mode_str),
        entry.name,
        state_label,
    );

    let open_item = MenuItem::with_id(
        app,
        format!("open:{}:{}", mode_str, entry.name),
        t(lang, "open-window"),
        true,
        None::<&str>,
    )?;

    let action_item = match entry.state {
        Some(InstanceState::Running) | Some(InstanceState::Starting) => MenuItem::with_id(
            app,
            format!("stop:{}:{}", mode_str, entry.name),
            t(lang, "action-stop"),
            true,
            None::<&str>,
        )?,
        _ => MenuItem::with_id(
            app,
            format!("start:{}:{}", mode_str, entry.name),
            t(lang, "action-start"),
            true,
            None::<&str>,
        )?,
    };

    Submenu::with_items(app, title, true, &[&open_item, &action_item])
}

fn collect_entries(app: &AppHandle) -> Vec<ConfigEntry> {
    let mut by_key: HashMap<String, ConfigEntry> = HashMap::new();

    // Start with on-disk configs (source of truth for "what configs exist").
    for mode in [Mode::Server, Mode::Client] {
        if let Ok(list) = store::list_configs(mode) {
            for cfg in list {
                let key = format!("{}:{}", mode.as_str(), cfg.name);
                by_key.insert(
                    key,
                    ConfigEntry {
                        mode,
                        name: cfg.name,
                        state: None,
                    },
                );
            }
        }
    }

    // Overlay running state from ProcessManager (and include any adopted
    // instances whose config file might no longer exist on disk).
    if let Some(mgr) = app.try_state::<Arc<ProcessManager>>() {
        for run in mgr.list() {
            let mode = match run.mode.as_str() {
                "server" => Mode::Server,
                "client" => Mode::Client,
                _ => continue,
            };
            let key = format!("{}:{}", run.mode, run.name);
            by_key
                .entry(key)
                .and_modify(|e| e.state = Some(run.state))
                .or_insert(ConfigEntry {
                    mode,
                    name: run.name,
                    state: Some(run.state),
                });
        }
    }

    let mut entries: Vec<ConfigEntry> = by_key.into_values().collect();
    // Sort: server first, client second; running first within each mode; then by name.
    entries.sort_by(|a, b| {
        let mode_ord = a.mode.as_str().cmp(b.mode.as_str());
        if mode_ord != std::cmp::Ordering::Equal {
            return mode_ord;
        }
        let a_running = matches!(
            a.state,
            Some(InstanceState::Running) | Some(InstanceState::Starting)
        );
        let b_running = matches!(
            b.state,
            Some(InstanceState::Running) | Some(InstanceState::Starting)
        );
        match (a_running, b_running) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });
    entries
}

fn handle_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    let id = event.id.as_ref();
    match id {
        "show-window" => show_window(app),
        "quit-app" => quit_app(app),
        _ => {
            if let Some((action, mode_str, name)) = parse_action_id(id) {
                let mode = match mode_str {
                    "server" => Mode::Server,
                    "client" => Mode::Client,
                    _ => return,
                };
                match action {
                    "open" => {
                        show_window(app);
                        let _ = app.emit(
                            "tray-navigate",
                            json!({ "mode": mode_str, "name": name }),
                        );
                    }
                    "start" => {
                        let app_clone = app.clone();
                        let name_owned = name.to_string();
                        tauri::async_runtime::spawn(async move {
                            handle_start(&app_clone, mode, name_owned).await;
                        });
                    }
                    "stop" => {
                        let app_clone = app.clone();
                        let name_owned = name.to_string();
                        tauri::async_runtime::spawn(async move {
                            handle_stop(&app_clone, mode, name_owned).await;
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn parse_action_id(id: &str) -> Option<(&str, &str, &str)> {
    let mut parts = id.splitn(3, ':');
    let action = parts.next()?;
    let mode = parts.next()?;
    let name = parts.next()?;
    Some((action, mode, name))
}

async fn handle_start(app: &AppHandle, mode: Mode, name: String) {
    match crate::commands::start_config_internal(app, mode, name.clone()).await {
        Ok(_) => {
            let _ = app.emit(
                "tray-action-result",
                json!({ "action": "start", "mode": mode.as_str(), "name": name, "ok": true }),
            );
        }
        Err(e) => {
            let _ = app.emit(
                "tray-action-result",
                json!({
                    "action": "start",
                    "mode": mode.as_str(),
                    "name": name,
                    "ok": false,
                    "error": e.to_string()
                }),
            );
        }
    }
}

async fn handle_stop(app: &AppHandle, mode: Mode, name: String) {
    match crate::commands::stop_config_internal(app, mode, &name).await {
        Ok(_) => {
            let _ = app.emit(
                "tray-action-result",
                json!({ "action": "stop", "mode": mode.as_str(), "name": name, "ok": true }),
            );
        }
        Err(e) => {
            let _ = app.emit(
                "tray-action-result",
                json!({
                    "action": "stop",
                    "mode": mode.as_str(),
                    "name": name,
                    "ok": false,
                    "error": e.to_string()
                }),
            );
        }
    }
}

fn handle_tray_icon_event(tray: &TrayIcon<Wry>, event: TrayIconEvent) {
    if let TrayIconEvent::DoubleClick {
        button: MouseButton::Left,
        ..
    } = event
    {
        show_window(tray.app_handle());
    }
}

fn show_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn quit_app(app: &AppHandle) {
    if let Some(mgr) = app.try_state::<Arc<ProcessManager>>() {
        let mgr = mgr.inner().clone();
        tauri::async_runtime::block_on(async move {
            mgr.stop_all().await;
        });
    }
    app.exit(0);
}

fn t(lang: &str, key: &str) -> &'static str {
    match (key, lang) {
        ("show", "zh-CN") => "显示窗口",
        ("show", "ja") => "ウィンドウを表示",
        ("show", "ko") => "창 표시",
        ("show", _) => "Show window",

        ("quit", "zh-CN") => "退出 Rathole Launcher",
        ("quit", "ja") => "Rathole Launcher を終了",
        ("quit", "ko") => "Rathole Launcher 종료",
        ("quit", _) => "Quit Rathole Launcher",

        ("no-configs", "zh-CN") => "尚无配置",
        ("no-configs", "ja") => "設定がまだありません",
        ("no-configs", "ko") => "구성이 없음",
        ("no-configs", _) => "No configurations yet",

        ("running-prefix", "zh-CN") => "运行中：",
        ("running-prefix", "ja") => "実行中：",
        ("running-prefix", "ko") => "실행 중：",
        ("running-prefix", _) => "Running:",

        ("open-window", "zh-CN") => "打开窗口",
        ("open-window", "ja") => "ウィンドウで開く",
        ("open-window", "ko") => "창에서 열기",
        ("open-window", _) => "Open in window",

        ("action-start", "zh-CN") => "启动",
        ("action-start", "ja") => "起動",
        ("action-start", "ko") => "시작",
        ("action-start", _) => "Start",

        ("action-stop", "zh-CN") => "停止",
        ("action-stop", "ja") => "停止",
        ("action-stop", "ko") => "중지",
        ("action-stop", _) => "Stop",

        ("state-running", "zh-CN") => "运行中",
        ("state-running", "ja") => "実行中",
        ("state-running", "ko") => "실행 중",
        ("state-running", _) => "running",

        ("state-starting", "zh-CN") => "启动中",
        ("state-starting", "ja") => "起動中",
        ("state-starting", "ko") => "시작 중",
        ("state-starting", _) => "starting",

        ("state-stopping", "zh-CN") => "停止中",
        ("state-stopping", "ja") => "停止中",
        ("state-stopping", "ko") => "중지 중",
        ("state-stopping", _) => "stopping",

        ("state-stopped", "zh-CN") => "已停止",
        ("state-stopped", "ja") => "停止済み",
        ("state-stopped", "ko") => "중지됨",
        ("state-stopped", _) => "stopped",

        _ => "?",
    }
}

fn mode_label(lang: &str, mode: &str) -> &'static str {
    match (lang, mode) {
        ("zh-CN", "server") => "服务端",
        ("zh-CN", "client") => "客户端",
        ("ja", "server") => "サーバー",
        ("ja", "client") => "クライアント",
        ("ko", "server") => "서버",
        ("ko", "client") => "클라이언트",
        (_, "server") => "Server",
        (_, "client") => "Client",
        _ => "?",
    }
}
