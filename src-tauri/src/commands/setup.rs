use std::process::Command;
use tauri::{async_runtime, AppHandle};

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
