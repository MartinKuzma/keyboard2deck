use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::result::Result;

const KEY_ERR_OVF: u8 = 0x01;

#[derive(Debug, PartialEq)]
pub enum KeyEventType {
    PRESSED,
    RELEASED,
}

#[derive(Debug)]
pub struct KeyEvent {
    pub key: Key,
    pub event_type: KeyEventType,
}

pub struct Keyboard {
    previous_events: Vec<Key>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            previous_events: Vec::with_capacity(6),
        }
    }

    pub fn events(&mut self, buf: &[u8; 8], length: usize) -> Result<Vec<KeyEvent>, &str> {
        let mut events = Vec::new();

        for n in 2..length {
            let value = buf[n];

            if value <= KEY_ERR_OVF {
                // Overflow, do nothing
                break;
            }

            let key = match Key::try_key_from(value) {
                Ok(k) => k,
                Err(_) => {
                    println!("Unknown key with value {}", value);
                    continue;
                }
            };

            print!("Key {:?} ",key);
            

            if self.map_previous_state(&key) {
                continue;
            }

            events.push(KeyEvent {
                key: key,
                event_type: KeyEventType::PRESSED,
            })
        }

        // Check for released keys
        for val in self.previous_events.iter() {
            if *val == Key::Unknown {
                continue;
            }

            events.push(KeyEvent {
                key: val.clone(),
                event_type: KeyEventType::RELEASED,
            })
        }

        //Clear previous state
        self.previous_events.clear();
        //Set current keys as previous state
        for event in events.iter() {
            if event.event_type == KeyEventType::PRESSED {
                self.previous_events.push(event.key.clone());
            }
        }

        println!("B {:?}", self.previous_events);

        return Result::Ok(events);
    }

    fn map_previous_state(&mut self, key_code: &Key) -> bool {
        for val in self.previous_events.iter_mut() {
            if *key_code == *val {
                // Remove key from previous state
                *val = Key::Unknown;
                return true;
            }
        }

        return false;
    }
}

// const KEY_MOD_LCTRL: u8 = 0x01;
// const KEY_MOD_LSHIFT: u8 = 0x02;
// const KEY_MOD_LALT: u8 = 0x04;
// const KEY_MOD_LMETA: u8 = 0x08;
// const KEY_MOD_RCTRL: u8 = 0x10;
// const KEY_MOD_RSHIFT: u8 = 0x20;
// const KEY_MOD_RALT: u8 = 0x40;
// const KEY_MOD_RMETA: u8 = 0x80;
// fn mod_code_to_rdev_key(code: u8, dest: &mut Vec<rdev::Key>) {
//     dest.clear();
//     if code & KEY_MOD_LCTRL != 0 {
//         dest.push(rdev::Key::ControlLeft);
//     }
//     if code & KEY_MOD_RCTRL != 0 {
//         dest.push(rdev::Key::ControlRight);
//     }
//     if code & KEY_MOD_LSHIFT != 0 {
//         dest.push(rdev::Key::ShiftLeft);
//     }
//     // TODO: Better parsing of bit flags...
// }

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Hash)]
pub enum Key {
    Unknown,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Enter,
    Esc,
    Backspace,
    Tab,
    Space,
    Minus,
    Equal,
    LeftBrace,
    Rightbrace,
    Backslash,
    Hashtilde,
    Semicolon,
    Apostrophe,
    Grave,
    Comma,
    Dot,
    Slash,
    Capslock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    Alt,
    AltLeft,
    AltRight,
    Intlbackslash,
    Home,
    Insert,
    Delete,
    End,
    PrintScreen,
}

impl Key {
    fn try_key_from(value: u8) -> Result<Self, ()> {
        match value {
            0x04 => Ok(Key::A),
            0x05 => Ok(Key::B),
            0x06 => Ok(Key::C),
            0x07 => Ok(Key::D),
            0x08 => Ok(Key::E),
            0x09 => Ok(Key::F),
            0x0a => Ok(Key::G),
            0x0b => Ok(Key::H),
            0x0c => Ok(Key::I),
            0x0d => Ok(Key::J),
            0x0e => Ok(Key::K),
            0x0f => Ok(Key::L),
            0x10 => Ok(Key::M),
            0x11 => Ok(Key::N),
            0x12 => Ok(Key::O),
            0x13 => Ok(Key::P),
            0x14 => Ok(Key::Q),
            0x15 => Ok(Key::R),
            0x16 => Ok(Key::S),
            0x17 => Ok(Key::T),
            0x18 => Ok(Key::U),
            0x19 => Ok(Key::V),
            0x1a => Ok(Key::W),
            0x1b => Ok(Key::X),
            0x1c => Ok(Key::Y),
            0x1d => Ok(Key::Z),
            0x1e => Ok(Key::Num1),
            0x1f => Ok(Key::Num2),
            0x20 => Ok(Key::Num3),
            0x21 => Ok(Key::Num4),
            0x22 => Ok(Key::Num5),
            0x23 => Ok(Key::Num6),
            0x24 => Ok(Key::Num7),
            0x25 => Ok(Key::Num8),
            0x26 => Ok(Key::Num9),
            0x27 => Ok(Key::Num0),
            0x28 => Ok(Key::Enter),
            0x29 => Ok(Key::Esc),
            0x2a => Ok(Key::Backspace),
            0x2b => Ok(Key::Tab),
            0x2c => Ok(Key::Space),
            0x2d => Ok(Key::Minus),
            0x2e => Ok(Key::Equal),
            0x2f => Ok(Key::LeftBrace),
            0x30 => Ok(Key::Rightbrace),
            0x31 => Ok(Key::Backslash),
            0x32 => Ok(Key::Hashtilde),
            0x33 => Ok(Key::Semicolon),
            0x34 => Ok(Key::Apostrophe),
            0x35 => Ok(Key::Grave),
            0x36 => Ok(Key::Comma),
            0x37 => Ok(Key::Dot),
            0x38 => Ok(Key::Slash),
            0x39 => Ok(Key::Capslock),
            0x3a => Ok(Key::F1),
            0x3b => Ok(Key::F2),
            0x3c => Ok(Key::F3),
            0x3d => Ok(Key::F4),
            0x3e => Ok(Key::F5),
            0x3f => Ok(Key::F6),
            0x40 => Ok(Key::F7),
            0x41 => Ok(Key::F8),
            0x42 => Ok(Key::F9),
            0x43 => Ok(Key::F10),
            0x44 => Ok(Key::F11),
            0x45 => Ok(Key::F12),
            0x4A => Ok(Key::Home),
            0x49 => Ok(Key::Insert),
            0x4C => Ok(Key::Delete),
            0x4D => Ok(Key::End),
            0x46 => Ok(Key::PrintScreen),
            _ => Err(()),
        }
    }

    pub fn try_mod_from(value: u8) -> Result<Self, ()> {
        match value {
            0x02 => Ok(Key::ShiftLeft),
            0x20 => Ok(Key::ShiftRight),
            0x01 => Ok(Key::ControlLeft),
            0x10 => Ok(Key::ControlRight),
            0x04 => Ok(Key::AltLeft),
            0x40 => Ok(Key::AltRight),
            _ => Err(()),
        }
    }

    pub fn try_into_rdev(&self) -> Result<rdev::Key, ()> {
        match self {
            Key::A => Ok(rdev::Key::KeyA),
            Key::B => Ok(rdev::Key::KeyB),
            Key::C => Ok(rdev::Key::KeyC),
            Key::D => Ok(rdev::Key::KeyD),
            Key::E => Ok(rdev::Key::KeyE),
            Key::F => Ok(rdev::Key::KeyF),
            Key::G => Ok(rdev::Key::KeyG),
            Key::H => Ok(rdev::Key::KeyH),
            Key::I => Ok(rdev::Key::KeyI),
            Key::J => Ok(rdev::Key::KeyJ),
            Key::K => Ok(rdev::Key::KeyK),
            Key::L => Ok(rdev::Key::KeyL),
            Key::M => Ok(rdev::Key::KeyM),
            Key::N => Ok(rdev::Key::KeyN),
            Key::O => Ok(rdev::Key::KeyO),
            Key::P => Ok(rdev::Key::KeyP),
            Key::Q => Ok(rdev::Key::KeyQ),
            Key::R => Ok(rdev::Key::KeyR),
            Key::S => Ok(rdev::Key::KeyS),
            Key::T => Ok(rdev::Key::KeyT),
            Key::U => Ok(rdev::Key::KeyU),
            Key::V => Ok(rdev::Key::KeyV),
            Key::W => Ok(rdev::Key::KeyW),
            Key::X => Ok(rdev::Key::KeyX),
            Key::Y => Ok(rdev::Key::KeyY),
            Key::Z => Ok(rdev::Key::KeyZ),
            Key::Num1 => Ok(rdev::Key::Num1),
            Key::Num2 => Ok(rdev::Key::Num2),
            Key::Num3 => Ok(rdev::Key::Num3),
            Key::Num4 => Ok(rdev::Key::Num4),
            Key::Num5 => Ok(rdev::Key::Num5),
            Key::Num6 => Ok(rdev::Key::Num6),
            Key::Num7 => Ok(rdev::Key::Num7),
            Key::Num8 => Ok(rdev::Key::Num8),
            Key::Num9 => Ok(rdev::Key::Num9),
            Key::Num0 => Ok(rdev::Key::Num0),
            Key::Enter => Ok(rdev::Key::Return),
            Key::Esc => Ok(rdev::Key::Escape),
            Key::Backspace => Ok(rdev::Key::Backspace),
            Key::Tab => Ok(rdev::Key::Tab),
            Key::Space => Ok(rdev::Key::Space),
            Key::Minus => Ok(rdev::Key::Minus),
            Key::Equal => Ok(rdev::Key::Equal),
            Key::LeftBrace => Ok(rdev::Key::LeftBracket),
            Key::Rightbrace => Ok(rdev::Key::RightBracket),
            Key::Backslash => Ok(rdev::Key::BackSlash),
            Key::Hashtilde => Ok(rdev::Key::BackQuote),
            Key::Semicolon => Ok(rdev::Key::SemiColon),
            Key::Apostrophe => Ok(rdev::Key::Quote),
            Key::Grave => Ok(rdev::Key::IntlBackslash),
            Key::Intlbackslash => Ok(rdev::Key::IntlBackslash),
            Key::Comma => Ok(rdev::Key::Comma),
            Key::Dot => Ok(rdev::Key::Dot),
            Key::Slash => Ok(rdev::Key::Slash),
            Key::Capslock => Ok(rdev::Key::CapsLock),
            Key::F1 => Ok(rdev::Key::F1),
            Key::F2 => Ok(rdev::Key::F2),
            Key::F3 => Ok(rdev::Key::F3),
            Key::F4 => Ok(rdev::Key::F4),
            Key::F5 => Ok(rdev::Key::F5),
            Key::F6 => Ok(rdev::Key::F6),
            Key::F7 => Ok(rdev::Key::F7),
            Key::F8 => Ok(rdev::Key::F8),
            Key::F9 => Ok(rdev::Key::F9),
            Key::F10 => Ok(rdev::Key::F10),
            Key::F11 => Ok(rdev::Key::F11),
            Key::F12 => Ok(rdev::Key::F12),
            Key::ShiftLeft => Ok(rdev::Key::ShiftLeft),
            Key::ShiftRight => Ok(rdev::Key::ShiftRight),
            Key::ControlLeft => Ok(rdev::Key::ControlLeft),
            Key::ControlRight => Ok(rdev::Key::ControlRight),
            Key::AltLeft => Ok(rdev::Key::Alt),
            Key::AltRight => Ok(rdev::Key::Alt),
            Key::Alt => Ok(rdev::Key::Alt),
            Key::Delete => Ok(rdev::Key::Delete),
            Self::PrintScreen => Ok(rdev::Key::PrintScreen),
            _ => Err(()),
        }
    }
}
