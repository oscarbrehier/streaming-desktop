use crate::core::{
    agent::spawn_agent,
    tailscale::{check_tailscale_installation, connect_to_network, install_tailscale},
};
use serde::{Deserialize, Serialize};
use std::process::Command;
use tauri::{async_runtime, AppHandle, Emitter, Window};

#[derive(Clone, Serialize, Deserialize)]
struct SetupProgress {
    id: String,
    title: String,
    status: String,
    message: String,
}

#[tauri::command]
pub async fn bootstrap_library(app: AppHandle, window: Window) -> Result<(), String> {
    window
        .emit(
            "setup-progress",
            SetupProgress {
                id: "check_tailscale".to_string(),
                title: "Setting up secure access".to_string(),
                status: "running".to_string(),
                message: "Checking your connection security".to_string(),
            },
        )
        .ok();

    let tailscale_installed = check_tailscale_installation();

    if !tailscale_installed {
        window
            .emit(
                "setup-progress",
                SetupProgress {
                    id: "check_tailscale".to_string(),
                    title: "Setting up secure access".to_string(),
                    status: "running".to_string(),
                    message: "Installing secure networking tools".to_string(),
                },
            )
            .ok();

        install_tailscale(&app)?;
    }

    window
        .emit(
            "setup-progress",
            SetupProgress {
                id: "check_tailscale".to_string(),
                title: "Setting up secure access".to_string(),
                status: "success".to_string(),
                message: "Secure access is ready".to_string(),
            },
        )
        .ok();

    window
        .emit(
            "setup-progress",
            SetupProgress {
                id: "connect_vps".to_string(),
                title: "Connecting to private network".to_string(),
                status: "running".to_string(),
                message: "Connecting you to the private network".to_string(),
            },
        )
        .ok();

    connect_to_network()?;

    window
        .emit(
            "setup-progress",
            SetupProgress {
                id: "connect_vps".to_string(),
                title: "Connecting to private network".to_string(),
                status: "success".to_string(),
                message: "You are now connected".to_string(),
            },
        )
        .ok();

    window
        .emit(
            "setup-progress",
            SetupProgress {
                id: "start_agent".to_string(),
                title: "Starting streaming services".to_string(),
                status: "running".to_string(),
                message: "Starting required services".to_string(),
            },
        )
        .ok();

    spawn_agent(&app)?;

    window
        .emit(
            "setup-progress",
            SetupProgress {
                id: "start_agent".to_string(),
                title: "Starting streaming services".to_string(),
                status: "success".to_string(),
                message: "Everything is ready to stream".to_string(),
            },
        )
        .ok();

    Ok(())
}

#[tauri::command]
pub async fn setup_environment(app: AppHandle) -> Result<(), String> {
    crate::commands::tailscale::install_tailscale(app.clone()).await?;

    let _ = async_runtime::spawn_blocking(move || -> Result<(), String> {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            Command::new("tailscale")
                .arg("up")
                .status()
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

// #[tauri::command]
// pub fn start_backend() -> Result<(), String> {
//     // Select OS-specific backend binary
//     let backend_path = match resource_dir() {
//         Some(dir) => {
//             #[cfg(target_os = "windows")]
//             { dir.join("backend/windows/backend.exe") }
//             #[cfg(target_os = "macos")]
//             { dir.join("backend/macos/backend") }
//             #[cfg(target_os = "linux")]
//             { dir.join("backend/linux/backend") }
//         }
//         None => return Err("Could not find resource directory".into()),
//     };

//     // Spawn backend
//     Command::new(backend_path)
//         .spawn()
//         .map_err(|e| e.to_string())?;

//     Ok(())
// }
