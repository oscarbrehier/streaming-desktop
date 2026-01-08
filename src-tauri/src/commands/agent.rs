use std::net::TcpStream;

use crate::core::agent::{spawn_agent};
use tauri::{AppHandle};

#[tauri::command]
pub fn start_agent(app: AppHandle) -> Result<(), String> {

	spawn_agent(&app)?;
	Ok(())

}

#[tauri::command]
pub async fn get_backend_status() -> Result<bool, String> {

	let addr = "127.0.0.1:3002";
	Ok(TcpStream::connect(addr).is_ok())

}