// Basic Macro trait
pub mod shell;
pub mod shortcut;

pub trait Macro: Sync {
    fn execute(&self);
}
