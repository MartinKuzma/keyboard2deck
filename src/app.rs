use hidapi;
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;
use std::collections::HashMap;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

use crate::config;
use crate::config::Config;
use crate::device::Device;
use crate::macros::{self, shortcut::ShortCut};

pub struct App {
    hid_api: Arc<Mutex<hidapi::HidApi>>,
    devices: Vec<Device>,
}

impl App {
    pub fn new(configuration: Config) -> Result<App, ()> {
        let mut app = App {
            hid_api: Arc::new(Mutex::new(hidapi::HidApi::new().unwrap())),
            devices: Vec::new(),
        };

        app.init(configuration);

        Ok(app)
    }

    pub fn run(&mut self) -> Result<(), ()> {
        let mut threads = Vec::new();
        let running = Arc::new(AtomicBool::new(true));

        // Run devices in separate threads
        while self.devices.len() > 0 {
            let mut device = self.devices.pop().unwrap();
            let running = running.clone();
            threads.push(thread::spawn(move || {
                device.listen(running);
            }));
        }

        let mut signals = Signals::new(&[SIGINT]).unwrap();
        for _ in signals.forever() {
            println!("Closing application");
            running.store(false, Ordering::Relaxed);
            break;
        }

        for thread in threads.into_iter() {
            thread.join().unwrap();
        }

        Ok(())
    }

    fn init(&mut self, config: Config) {
        for conf_device in config.devices {
            let mut macros = HashMap::new();
            for conf_macro in conf_device.macros {
                let macr: Box<dyn macros::Macro + Send> = match conf_macro.oneof_macro {
                    config::OneOfMacros::Shell(shell_macro) => Box::new(shell_macro),
                    config::OneOfMacros::Shortcut(shortcut_config) => {
                        Box::new(ShortCut::new(shortcut_config.keys))
                    }
                };
                macros.insert(conf_macro.key.clone(), macr);
            }

            self.devices.push(Device::new(
                conf_device.vid,
                conf_device.pid,
                macros,
                self.hid_api.clone(),
            ));
        }
    }
}

pub fn list_devices() {
    let hid_api = hidapi::HidApi::new().unwrap();

    println!("Found HID USB devices:\n");

    for device in hid_api.device_list() {
        println!(
            "SN:{:#?}\tVID: {}\t PID: {}\t\tNAME: {}",
            device.serial_number().unwrap_or("N/A"),
            device.vendor_id(),
            device.product_id(),
            device.product_string().unwrap_or("N/A")
        );
    }
}
