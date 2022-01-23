use std::{collections::HashMap, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread};

use hidapi::{HidDevice, HidError};

use crate::{keyboard, macros::Macro};

pub type MacrosBinding = HashMap<keyboard::Key, Arc<dyn Macro + Send>>;

#[derive(Clone)]
pub struct DeviceConfig {
    pub vid: u16,
    pub pid: u16,
    pub macros: MacrosBinding,
}

pub struct Device {
    config: DeviceConfig,
    hid_api: Arc<Mutex<hidapi::HidApi>>,
}

impl Device {
    pub fn new(config: DeviceConfig, hid_api: Arc<Mutex<hidapi::HidApi>>) -> Device {
        Device {
            config: config,
            hid_api: hid_api,
        }
    }

    pub fn listen(&mut self, running: Arc<AtomicBool>) {
        let wait_duration = std::time::Duration::from_secs(5);

        while running.load(Ordering::Relaxed) {
            if !self.is_present() {
                thread::sleep(wait_duration);
                continue;
            }

            match self.open_device() {
                Ok(hid_device) => self.process_events(hid_device, &running),
                Err(_) => {
                    println!(
                        "cannot open device: {} {}",
                        self.config.vid, self.config.pid
                    );
                    thread::sleep(wait_duration);
                    continue;
                }
            }
        }
    }

    pub fn open_device(&mut self) -> Result<HidDevice, HidError> {
        let api = self.hid_api.lock().unwrap();
        api.open(self.config.vid, self.config.pid)
    }

    pub fn is_present(&mut self) -> bool {
        let mut api = self.hid_api.lock().unwrap();
        api.refresh_devices().unwrap();

        for device in api.device_list() {
            if device.vendor_id() == self.config.vid && device.product_id() == self.config.pid {
                return true;
            }
        }

        return false;
    }

    pub fn process_events(&self, hid_device: HidDevice, running: &Arc<AtomicBool>) {
        let mut keyboard = keyboard::Keyboard::new();

        while running.load(Ordering::Relaxed) {
            let mut buf = [0u8; 8];
            let res = hid_device.read_timeout(&mut buf[..], 2500).unwrap();

            if res == 0 {
                continue;
            }

            for event in keyboard.events(&buf, res).unwrap() {
                if event.event_type == keyboard::KeyEventType::PRESSED {
                    continue;
                }

                println!("{:?}", event.key);

                match self.config.macros.get(&event.key) {
                    Some(m) => m.execute(),
                    None => {}
                }
            }
        }
    }
}
