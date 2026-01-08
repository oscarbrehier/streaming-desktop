#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod utils;
mod core;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
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
            commands::setup::run_setup,
            commands::setup::setup_environment,
            commands::tailscale::get_tailscale_status,
            commands::agent::start_agent
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
