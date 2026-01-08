use reqwest::Client;
use serde::Deserialize;
use tauri::AppHandle;
use std::process::Command;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
pub struct TailscaleStatus {
    #[serde(rename = "BackendState")]
    backend_state: String,
}

impl TailscaleStatus {
    pub fn backend_state(&self) -> &str {
        &self.backend_state
    }
}

pub fn install_tailscale(app: &AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let pkg = download_file(
            &app,
            "https://pkgs.tailscale.com/stable/Tailscale-latest-macos.pkg",
            "Tailscale.pkg",
        )?;

        Command::new("installer")
            .args(["-pkg", pkg.to_str().unwrap(), "-target", "/"])
            .status()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        use crate::utils::download::download_file;

        let exe = download_file(
            &app,
            "https://pkgs.tailscale.com/stable/tailscale-setup-latest.exe",
            "tailscale-setup.exe",
        )?;

        Command::new(exe)
            .args(["/quiet"])
            .status()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn check_tailscale_installation() -> bool {
    #[cfg(target_os = "windows")]
    let cmd = Command::new("tailscale").arg("version").output();

    cmd.is_ok() && cmd.unwrap().status.success()
}

pub fn tailscale_status() -> Result<TailscaleStatus, String> {
    let output = Command::new("tailscale")
        .args(["status", "--json"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("Tailscale not running".into());
    }

    let status: TailscaleStatus =
        serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

    Ok(status)
}

pub fn tailscale_start() -> Result<(), String> {
    Command::new("tailscale")
        .arg("up")
        .status()
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn tailscale_stop() -> Result<(), String> {
    Command::new("tailscale")
        .arg("down")
        .status()
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn connect_to_network() -> Result<(), String> {

	let output = Command::new("tailscale")
		.args(["up", "--authkey", "authkey"])
		.output()
		.map_err(|e| e.to_string())?;

	if !output.status.success() {
		return Err("Failed to connect to VPS".to_string());
	}

	Ok(())

}

pub async fn check_vps_connection() -> bool {

    dotenv().ok();

    let base_path = std::env::var("STREAMING_APP_URL").unwrap_or("".to_string());
    let url = format!("{}/api/health", base_path);

    Client::new()
        .get(url)
        .send()
        .await
        .map(|resp| resp.status().is_success())
        .unwrap_or(false)

}