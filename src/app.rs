use hidapi;
use hidapi::HidError;
use std::{
    collections::HashMap,
    fs::{self},
    sync::Arc,
    thread,
};
use yaml_rust::yaml;

use crate::{
    keyboard,
    macros::{self, shell::ShellMacro, shortcut::ShortCut, Macro},
};

type MacrosBinding = HashMap<keyboard::Key, Arc<dyn Macro + Send>>;

#[derive(Clone)]
pub struct Device {
    pub vid: u16,
    pub pid: u16,
    pub macros: MacrosBinding,
}

pub struct App {
    hid_api: Arc<hidapi::HidApi>,
    devices: Vec<Device>,
}

impl App {
    pub fn new(configuration: String) -> Result<App, HidError> {
        let content = fs::read_to_string(configuration).unwrap();
        let mut app = App {
            hid_api: Arc::new(hidapi::HidApi::new().unwrap()),
            devices: Vec::new(),
        };

        app.parse_config(content).unwrap();
        Ok(app)
    }

    pub fn run(&mut self) -> Result<(), &str> {
        let mut threads = Vec::new();

        for device in self.devices.iter() {
            let api = self.hid_api.clone();
            let cloned_device = device.clone();

            threads.push(thread::spawn(move || {
                App::listen_to_device(api, cloned_device);
            }));
        }

        for thread in threads.into_iter() {
            thread.join().unwrap();
        }

        Ok(())
    }

    fn listen_to_device(api: Arc<hidapi::HidApi>, device: Device) {
        let hid_device = api.open(device.vid, device.pid).unwrap();
        let mut keyboard = keyboard::Keyboard::new();

        loop {
            let mut buf = [0u8; 8];
            let res = hid_device.read(&mut buf[..]).unwrap();

            for event in keyboard.events(&buf, res).unwrap() {
                if event.event_type == keyboard::KeyEventType::PRESSED {
                    continue;
                }

                match device.macros.get(&event.key) {
                    Some(m) => m.execute(),
                    None => {}
                }
            }
        }
    }

    fn parse_config(&mut self, config: String) -> Result<(), &str> {
        let docs = yaml_rust::YamlLoader::load_from_str(&config).unwrap();
        let doc = &docs[0];

        let devices_yml = doc["devices"].as_vec().unwrap();

        for device_yml in devices_yml.iter() {
            let device = Device {
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
            let macro_type = m["type"].as_str().unwrap();

            let parsed_macro: Arc<dyn macros::Macro + Send> = match macro_type {
                "shell" => Arc::new(ShellMacro {
                    command: String::from(m["command"].as_str().unwrap()),
                }),
                "shortcut" => Arc::new(
                    ShortCut::new(
                        m["keys"]
                            .as_vec()
                            .unwrap()
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
