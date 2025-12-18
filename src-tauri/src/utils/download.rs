use std::{fs::File, io::{self, Read}, path::PathBuf};
use tauri::{AppHandle, Manager}; // ← Manager needed for path()

pub fn download_file(app: &AppHandle, url: &str, filename: &str) -> Result<PathBuf, String> {

    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| e.to_string())?;
    let dir = cache_dir.join("streaming");

    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let path = dir.join(filename);

    let mut response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Download failed: {}", response.status()));
    }

    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();

    response.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    io::copy(&mut &buffer[..], &mut file).map_err(|e| e.to_string())?;

    Ok(path)
}
