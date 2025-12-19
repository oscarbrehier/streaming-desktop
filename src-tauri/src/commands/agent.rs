use crate::core::agent::{spawn_agent};
use tauri::{AppHandle};

#[tauri::command]
pub fn start_agent(app: AppHandle) -> Result<(), String> {

	spawn_agent(&app)?;
	Ok(())

}