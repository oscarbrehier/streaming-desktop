use crate::utils::download::download_file;
use std::process::Command;
use tauri::{async_runtime, AppHandle};

#[tauri::command]
pub async fn install_tailscale(app: AppHandle) -> Result<(), String> {
    async_runtime::spawn_blocking(move || {
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
    })
	.await
	.map_err(|e| e.to_string())?
}
