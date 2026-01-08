use std::process::Command;
use tauri::{AppHandle, Manager};

pub fn spawn_agent(app: &AppHandle) -> Result<(), String> {

    #[cfg(target_os = "windows")]
    let resource_path = "windows/agent.exe";
	
	#[cfg(target_os = "macos")]
    let resource_path = "macos/agent";
	
	#[cfg(target_os = "linux")]
    let resource_path = "linux/agent";

	let agent_path = match app.path().resolve(resource_path, tauri::path::BaseDirectory::Resource) {
		Ok (path) if path.exists() => {
			path
		},
		_ => {

			let path = std::env::current_dir()
				.map_err(|e| e.to_string())?
				.join("resources")
				.join(resource_path);
            
            if !path.exists() {
                return Err(format!("Agent not found in either location. Tried:\n- Resource dir\n- {:?}", path));
            }
            path
		}
	};

	Command::new(agent_path)
		.spawn()
		.map_err(|e| e.to_string())?;

	Ok(())
}