use std::{ffi::OsStr, process::Command};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// Helper to prevent windows from opening a terminal window
pub fn create_hidden_command<S: AsRef<OsStr>>(program: S) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000);
    }
    cmd
}