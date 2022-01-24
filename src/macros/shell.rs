use serde::Deserialize;

use super::Macro;
use std::process::Command;

// Macro capable of running shell commands
#[derive(Deserialize)]
pub struct ShellMacro {
    pub command: String,
    #[cfg(any(target_os = "linux"))]
    pub uid: u32,
    pub args: Vec<String>,
}

impl Macro for ShellMacro {
    #[cfg(any(target_os = "linux"))]
    fn execute(&self) {
        use std::{os::unix::process::CommandExt, process::Stdio};

        let _ = Command::new(&self.command)
            .uid(self.uid)
            .args(&self.args)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    }

    #[cfg(any(target_os = "windows"))]
    fn execute(&self) {
        let _ = Command::new(self.command).args(self.args).spawn();
    }
}

