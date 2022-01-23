use rdev::{simulate, EventType, SimulateError};
use std::io::Error;

use super::Macro;
use crate::keyboard;

pub struct ShortCut {
    keys: Vec<rdev::Key>,
}

impl Macro for ShortCut {
    fn execute(&self) {
        for key in self.keys.iter() {
            ShortCut::send(&EventType::KeyPress(*key));
        }

        for key in self.keys.iter() {
            ShortCut::send(&EventType::KeyRelease(*key));
        }
    }
}

impl ShortCut {
    pub fn new(keys: Vec<String>) -> Result<ShortCut, Error> {
        let mut parsed_keys: Vec<rdev::Key> = Vec::new();

        for key_str in keys.iter() {
            let key: keyboard::Key = match serde_yaml::from_str(&key_str) {
                Ok(k) => k,
                Err(_) => panic!("Error while parsing key for shortcuts macro: {}", key_str),
            };

            match key.try_into_rdev() {
                Ok(k) => parsed_keys.push(k),
                Err(_) => panic!("Unsupported key {}", key_str),
            }
        }

        return Ok(ShortCut { keys: parsed_keys });
    }

    fn send(event_type: &EventType) {
        let delay = std::time::Duration::from_millis(20);
        match simulate(event_type) {
            Ok(()) => (),
            Err(SimulateError) => {
                println!("Error during sending key: {:?}", event_type);
            }
        }
        std::thread::sleep(delay);
    }
}
