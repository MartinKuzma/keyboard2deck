use rdev::{simulate, EventType, SimulateError};

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
    pub fn new(keys: Vec<keyboard::Key>) -> ShortCut {
        let mut parsed_keys: Vec<rdev::Key> = Vec::new();

        for key in keys.iter() {
            match key.try_into_rdev() {
                Ok(k) => parsed_keys.push(k),
                Err(_) => panic!("Unsupported key in configuration"),
            }
        }

        return ShortCut { keys: parsed_keys };
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
