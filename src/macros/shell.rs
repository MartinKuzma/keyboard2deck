use serde::Deserialize;

use super::Macro;
use std::{collections::HashMap, process::Command};

// Macro capable of running shell commands
#[derive(Deserialize)]
pub struct ShellMacro {
    pub command: String,
    #[cfg(any(target_os = "linux"))]
    pub uid: u32,
    pub args: Vec<String>,
    pub envs: Option<HashMap<String, String>>,
}

impl Macro for ShellMacro {
    #[cfg(any(target_os = "linux"))]
    fn execute(&self) {
        use std::os::unix::process::CommandExt;

        let mut command = Command::new(&self.command);
        command.args(&self.args);
        command.uid(self.uid);
        // .stdout(Stdio::null())
        // .stderr(Stdio::null())
    
        if let Some(envs) = &self.envs {
            command.envs(envs);
        }

        let _ = command.spawn();
    }

    #[cfg(any(target_os = "windows"))]
    fn execute(&self) {
        let _ = Command::new(&self.command)
            .args(&self.args)
            .spawn();
    }
}
