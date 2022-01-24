
use serde;
use serde::Deserialize;

use crate::keyboard;
use crate::macros::shell::ShellMacro;

#[derive(Deserialize)]
pub struct Shortcut {
    pub keys : Vec<keyboard::Key>
}

#[derive(Deserialize)]
pub enum OneOfMacros {
    #[serde(rename = "shell")]
    Shell(ShellMacro), 
    #[serde(rename = "shortcut")]
    Shortcut(Shortcut)
}

#[derive(Deserialize)]
pub struct Macro {
    pub key: keyboard::Key,
    #[serde(flatten)]
    pub oneof_macro : OneOfMacros,
}

#[derive(Deserialize)]
pub struct DeviceConfiguration {
    pub vid: u16,
    pub pid: u16,
    pub macros: Vec<Macro>,
}

#[derive(Deserialize)]
pub struct Config {
    pub devices: Vec<DeviceConfiguration>,
}
