use std::{process::Command, sync::Mutex};
use tauri::{AppHandle, Manager};

use crate::{AppState, utils::utils::create_hidden_command};

pub fn spawn_agent(app: &AppHandle) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let resource_path = "windows/agent.exe";

    #[cfg(target_os = "macos")]
    let resource_path = "macos/agent";

    #[cfg(target_os = "linux")]
    let resource_path = "linux/agent";

    let agent_path = match app
        .path()
        .resolve(resource_path, tauri::path::BaseDirectory::Resource)
    {
        Ok(path) if path.exists() => path,
        _ => {
            let path = std::env::current_dir()
                .map_err(|e| e.to_string())?
                .join("resources")
                .join(resource_path);

            if !path.exists() {
                return Err(format!(
                    "Agent not found in either location. Tried:\n- Resource dir\n- {:?}",
                    path
                ));
            }
            path
        }
    };

    let child = create_hidden_command(agent_path)
        .spawn()
        .map_err(|e| e.to_string())?;

    let state_handle = app.state::<Mutex<AppState>>();
    let mut state = state_handle.lock().unwrap();

    if let Some(mut old_child) = state.agent.take() {
        let _ = old_child.kill();
    }

    state.agent = Some(child);

    Ok(())
}

pub fn kill_agent_process(app: &AppHandle) -> Result<(), String> {
    let state_handle = app.state::<Mutex<AppState>>();
    let mut state = state_handle.lock().unwrap();

    if let Some(mut child) = state.agent.take() {
        child.kill().map_err(|e| e.to_string())?;
        let _ = child.wait();
    } else {
        return Err("No agent was running".to_string());
    }

    Ok(())
}
