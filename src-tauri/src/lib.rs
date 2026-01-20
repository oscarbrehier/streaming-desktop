#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod core;
mod utils;
use std::{process::Child, sync::Mutex};

use tauri::Manager;

#[derive(Default)]
struct AppState {
    agent: Option<Child>
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::tailscale::install_tailscale,
            commands::setup::bootstrap_library,
            commands::setup::setup_environment,
            commands::tailscale::get_tailscale_status,
            commands::tailscale::get_vps_status,
            commands::agent::start_agent,
            commands::agent::terminate_agent,
            commands::agent::get_backend_status,
        ])
        .on_window_event(|window, event| {

            if let tauri::WindowEvent::Destroyed = event {
                let state = window.state::<Mutex<AppState>>();
                let mut lock = state.lock().unwrap();
                
                if let Some(mut child) = lock.agent.take() {
                    let _ = child.kill();
                }
            }

        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
