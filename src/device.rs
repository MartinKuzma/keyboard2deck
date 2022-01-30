use crate::{keyboard, macros::Macro};
use hidapi::{HidDevice, HidError};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

pub type MacrosBinding = HashMap<keyboard::Key, Box<dyn Macro + Send>>;

pub struct Device {
    vid: u16,
    pid: u16,
    macros: MacrosBinding,
    hid_api: Arc<Mutex<hidapi::HidApi>>,
}

impl Device {
    pub fn new(
        vid: u16,
        pid: u16,
        macros: MacrosBinding,
        hid_api: Arc<Mutex<hidapi::HidApi>>,
    ) -> Device {
        Device {
            vid: vid,
            pid: pid,
            macros: macros,
            hid_api: hid_api,
        }
    }

    pub fn listen(&mut self, stop: Arc<AtomicBool>) {
        let wait_duration = std::time::Duration::from_secs(5);

        while !stop.load(Ordering::Relaxed) {
            if !self.is_present() {
                thread::sleep(wait_duration);
                continue;
            }

            if let Ok(hid_device) = self.open_device() {
                match self.process_events(hid_device, &stop) {
                    Ok(_) => continue,
                    Err(_) =>{ 
                        println!("Communication error. Device will not be listened to anymore.");
                        return;
                    }
                }
            } else {
                println!("cannot open device: {} {}", self.vid, self.pid);
                thread::sleep(wait_duration);
                continue;
            }
        }
    }

    pub fn open_device(&mut self) -> Result<HidDevice, HidError> {
        let api = self.hid_api.lock().unwrap();
        api.open(self.vid, self.pid)
        //api.open_serial(vid, pid, sn)
    }

    pub fn is_present(&mut self) -> bool {
        let mut api = self.hid_api.lock().unwrap();
        api.refresh_devices().unwrap();

        for device in api.device_list() {
            if device.vendor_id() == self.vid && device.product_id() == self.pid {
                return true;
            }
        }

        return false;
    }

    pub fn process_events(&self, hid_device: HidDevice, stop: &Arc<AtomicBool>) -> Result<(), HidError> {
        let mut keyboard = keyboard::Keyboard::new();

        while !stop.load(Ordering::Relaxed) {
            let mut buf = [0u8; 18];
            let res = match hid_device.read_timeout(&mut buf[..], 2500)  {
                Ok(r) => r,
                Err(e) => {
                    println!("Error while reading from device: {}", e);
                    return Err(e);
                }
            };

            if res == 0 {
                continue;
            }

            for event in keyboard.events(&buf, res) {
                if event.event_type == keyboard::KeyEventType::PRESSED {
                    continue;
                }

                match self.macros.get(&event.key) {
                    Some(m) => m.execute(),
                    None => {}
                }
            }
        }

        Ok(())
    }
}
