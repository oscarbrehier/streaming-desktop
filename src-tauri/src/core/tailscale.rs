use serde::Deserialize;
use std::process::Command;

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