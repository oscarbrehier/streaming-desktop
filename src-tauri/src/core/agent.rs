use std::process::Command;
use tauri::{AppHandle, Manager};

pub fn spawn_agent(app: &AppHandle) -> Result<(), String> {
    let resource_dir = app.path().resource_dir().map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    let agent_path = resource_dir.join("windows/agent.exe");
	
	#[cfg(target_os = "macos")]
    let agent_path = resource_dir.join("macos/agent");
	
	#[cfg(target_os = "linux")]
    let agent_path = resource_dir.join("linux/agent");

	Command::new(agent_path)
		.spawn()
		.map_err(|e| e.to_string())?;

	Ok(())
}
