use crate::core::tailscale;
use crate::utils::download::download_file;
use crate::utils::utils::create_hidden_command;
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
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_tailscale_status() -> Result<String, String> {
    let status = tauri::async_runtime::spawn_blocking(|| -> Result<String, String> {
        let status = tailscale::tailscale_status()?;
        Ok(status.backend_state().to_string())
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(status)
}

#[tauri::command]
pub async fn get_vps_status() -> Result<bool, String> {
    Ok(tailscale::check_vps_connection().await)
}
