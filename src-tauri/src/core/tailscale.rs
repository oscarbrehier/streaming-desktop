use crate::utils::download::download_file;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use tauri::AppHandle;
use crate::utils::utils::create_hidden_command;

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

        create_hidden_command("installer")
            .args(["-pkg", pkg.to_str().unwrap(), "-target", "/"])
            .status()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        let exe = download_file(
            &app,
            "https://pkgs.tailscale.com/stable/tailscale-setup-latest.exe",
            "tailscale-setup.exe",
        )?;

        create_hidden_command(exe)
            .args(["/quiet"])
            .status()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn check_tailscale_installation() -> bool {
    let cmd = create_hidden_command("tailscale").arg("version").output();

    cmd.is_ok() && cmd.unwrap().status.success()
}

pub fn tailscale_status() -> Result<TailscaleStatus, String> {
    let output = create_hidden_command("tailscale")
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

// pub fn tailscale_start() -> Result<(), String> {
//     Command::new("tailscale")
//         .arg("up")
//         .status()
//         .map_err(|e| e.to_string())?;

//     Ok(())
// }

// pub fn tailscale_stop() -> Result<(), String> {
//     Command::new("tailscale")
//         .arg("down")
//         .status()
//         .map_err(|e| e.to_string())?;

//     Ok(())
// }

pub fn connect_to_network() -> Result<(), String> {
    let oauth_secret = std::env::var("TAILSCALE_CLIENT_SECRET").unwrap_or("".to_string());
    let auth_key_with_flags = format!("{}?ephemeral=false&preauthorized=true", oauth_secret);

    let output = create_hidden_command("tailscale")
        .args([
            "up",
            "--authkey",
            &auth_key_with_flags,
            "--advertise-tags=tag:guest",
            "--reset",
        ])
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
