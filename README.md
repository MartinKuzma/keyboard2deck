# keyboard2deck
Turn your old keyboards into customizable decks. Assign shortcuts or command to various keys and improve your productivity.

Project was created after I became fatiqued by repetitive actions. Dedicated streaming decks however seemed a bit expensive solution, especially considering that I already have functioning keyboards I don't use. Application si written in Rust.

## Features
- List HID USB Devices
- Setup different macros on multiple devices
- Shortcuts (emits simultanously pressed keys)
- Sequential output (emits gradually pressed keys)
- Shell commands
- Currently mainly for Linux (Windows support will be added later)

## Planned features
- Better error handling and general code improvements
- Proper thread handling and termination
- Clipboard copy/paste with multiple memory buffers
- Clipboard paste with transformation rules
- Sequential key output (different from Shortcuts)
- Add modifier keys support
- Consolidate two different sets of keys

How to run:
```bash
cargo build
#list all devices
./keyboard2deck -l 
#find out which device you are interested in and set it in configuration file
#run as service
./keyboard2deck -c config.yaml
```

## Example configuration
```yaml
---
devices:
  - vid: 6127
    pid: 24647
    macros:
      - key: "A"
        type: shell
        command: "chromium"
      - key: "B"
        description: "Runs Ctrl+Alt+Delete"
        type: shortcut
        keys:
          - "ControlLeft"
          - "Alt"
          - "Delete"
      - key: "F"
        description: "Runs format code in VSCode"
        type: shortcut
        keys:
          - "ControlLeft"
          - "ShiftLeft"
          - "I"
```

## Supported keys

Configuration file is currently case-sensitive.

```
A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9, Num0, Enter, Esc, Backspace, Tab, Space, Minus, Equal, LeftBrace, Rightbrace, Backslash, Hashtilde, Semicolon, Apostrophe, Grave, Comma, Dot, Slash, Capslock, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, ShiftLeft, ShiftRight, ControlLeft, ControlRight, Alt, AltLeft, AltRight, Intlbackslash, Home, Insert, Delete, End, PrintScreen,      
```