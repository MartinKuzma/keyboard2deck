use hidapi::{self, HidDevice};
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::{
    collections::HashMap,
    fs::{self},
    sync::Arc,
    thread,
};
use yaml_rust::yaml;

use crate::device;
use crate::device::MacrosBinding;
use crate::device::Device;
use crate::device::DeviceConfig;
use crate::{
    macros::{self, shell::ShellMacro, shortcut::ShortCut, Macro},
};

pub struct App {
    hid_api: Arc<Mutex<hidapi::HidApi>>,
    devices: Vec<DeviceConfig>,
}

impl App {
    pub fn new(configuration: String) -> Result<App, ()> {
        let content = fs::read_to_string(configuration).unwrap();

        let mut app = App {
            hid_api: Arc::new(Mutex::new(hidapi::HidApi::new().unwrap())),
            devices: Vec::new(),
        };

        app.parse_config(content).unwrap();
        Ok(app)
    }

    pub fn run(&mut self) -> Result<(), ()> {
        let mut threads = Vec::new();
        let running = Arc::new(AtomicBool::new(true));

        // Run devices in separate threads
        while self.devices.len() > 0 {
            let device_config = self.devices.pop().unwrap();
            let api = self.hid_api.clone();
            let device_running = running.clone();
            threads.push(thread::spawn(move || {
                Device::new(device_config, api).listen(device_running);
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

    fn parse_config(&mut self, config: String) -> Result<(), &str> {
        let docs = yaml_rust::YamlLoader::load_from_str(&config).unwrap();
        let doc = &docs[0];

        let devices_yml = doc["devices"].as_vec().unwrap();

        for device_yml in devices_yml.iter() {
            let device = DeviceConfig {
                vid: u16::try_from(device_yml["vid"].as_i64().unwrap())
                    .expect("Cannot parse vid value in device element!"),
                pid: u16::try_from(device_yml["pid"].as_i64().unwrap())
                    .expect("Cannot parse pid value in device element!"),
                macros: App::parse_macros(&device_yml["macros"]),
            };
            self.devices.push(device);
        }

        Ok(())
    }

    fn parse_macros(macros_yml: &yaml::Yaml) -> MacrosBinding {
        let mut macros_binding = MacrosBinding::new();

        for m in macros_yml.as_vec().unwrap().iter() {
            let key = serde_yaml::from_str(m["key"].as_str().unwrap()).unwrap();
            let macro_type = m["type"].as_str().expect("Missing type in macros.");

            let parsed_macro: Arc<dyn macros::Macro + Send> = match macro_type {
                "shell" => Arc::new(ShellMacro {
                    command: String::from(m["command"].as_str().unwrap()),
                }),
                "shortcut" => Arc::new(
                    ShortCut::new(
                        m["keys"]
                            .as_vec()
                            .expect("Missing keys in element")
                            .iter()
                            .map(|v| String::from(v.as_str().unwrap()))
                            .collect(),
                    )
                    .unwrap(),
                ),
                unknown => panic!("Incorrect macro type: {}", unknown),
            };

            macros_binding.insert(key, parsed_macro);
        }

        macros_binding
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
