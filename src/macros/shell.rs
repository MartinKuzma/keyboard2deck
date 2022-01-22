use std::process::Command;

use super::Macro;

// Macro capable of running shell commands
pub struct ShellMacro {
    pub command: String,
}

impl Macro for ShellMacro {
    fn execute(&self) {
        let _ = Command::new("sh")
            .arg("-c")
            .arg(self.command.as_str())
            .spawn();
    }
}
