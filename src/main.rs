extern crate winapi;

use std::env;
use std::ptr::null_mut;
use std::thread;
use std::time::Duration;
use winapi::shared::minwindef::{DWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::um::winuser::{
  CallNextHookEx, DispatchMessageW, GetMessageW, PostQuitMessage, SetWindowsHookExW,
  TranslateMessage, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN,
};

use once_cell::sync::Lazy;
use std::collections::HashSet;
use indexmap::IndexMap;

static mut DISABLED_KEYS: Option<HashSet<i32>> = None;

static KEY_MAP: Lazy<IndexMap<&'static str, i32>> = Lazy::new(|| {
  let mut key_map = IndexMap::new();
  key_map.extend([
    ("A", 0x41), ("B", 0x42), ("C", 0x43), ("D", 0x44), ("E", 0x45),
    ("F", 0x46), ("G", 0x47), ("H", 0x48), ("I", 0x49), ("J", 0x4A),
    ("K", 0x4B), ("L", 0x4C), ("M", 0x4D), ("N", 0x4E), ("O", 0x4F),
    ("P", 0x50), ("Q", 0x51), ("R", 0x52), ("S", 0x53), ("T", 0x54),
    ("U", 0x55), ("V", 0x56), ("W", 0x57), ("X", 0x58), ("Y", 0x59),
    ("Z", 0x5A),

    ("0", 0x30), ("1", 0x31), ("2", 0x32), ("3", 0x33), ("4", 0x34),
    ("5", 0x35), ("6", 0x36), ("7", 0x37), ("8", 0x38), ("9", 0x39),

    ("F1", 0x70), ("F2", 0x71), ("F3", 0x72), ("F4", 0x73), ("F5", 0x74),
    ("F6", 0x75), ("F7", 0x76), ("F8", 0x77), ("F9", 0x78), ("F10", 0x79),
    ("F11", 0x7A), ("F12", 0x7B), ("F13", 0x7C), ("F14", 0x7D), ("F15", 0x7E),
    ("F16", 0x7F), ("F17", 0x80), ("F18", 0x81), ("F19", 0x82), ("F20", 0x83),
    ("F21", 0x84), ("F22", 0x85), ("F23", 0x86), ("F24", 0x87),

    ("ESC", 0x1B),
    ("SPACE", 0x20),
    ("ENTER", 0x0D),
    ("TAB", 0x09),
    ("BACKSPACE", 0x08),
    ("DELETE", 0x2E),
    ("INSERT", 0x2D),
    ("HOME", 0x24),
    ("END", 0x23),
    ("PAGE_UP", 0x21),
    ("PAGE_DOWN", 0x22),
    ("UP", 0x26),
    ("DOWN", 0x28),
    ("LEFT", 0x25),
    ("RIGHT", 0x27),
    ("CTRL", 0x11),
    ("SHIFT", 0x10),
    ("ALT", 0x12),
    ("WIN_L", 0x5B),
    ("WIN_R", 0x5C),
    ("APPS", 0x5D),
    ("MEDIA_PLAY_PAUSE", 0xB3),
    ("MEDIA_STOP", 0xB2),
    ("MEDIA_NEXT_TRACK", 0xB0),
    ("MEDIA_PREV_TRACK", 0xB1),
    ("VOLUME_UP", 0xAF),
    ("VOLUME_DOWN", 0xAE),
    ("VOLUME_MUTE", 0xAD),
    ("BROWSER_BACK", 0xA6),
    ("BROWSER_FORWARD", 0xA7),
    ("BROWSER_REFRESH", 0xA8),
    ("BROWSER_STOP", 0xA9),
    ("BROWSER_SEARCH", 0xAA),
    ("BROWSER_FAVORITES", 0xAB),
    ("BROWSER_HOME", 0xAC),
    ("PRINT_SCREEN", 0x2C),
    ("SCROLL_LOCK", 0x91),
    ("PAUSE", 0x13),
    ("SLEEP", 0x5F),
    ("POWER", 0x5E),
    ("WAKE", 0xE3),
    ("NUMPAD_0", 0x60),
    ("NUMPAD_1", 0x61),
    ("NUMPAD_2", 0x62),
    ("NUMPAD_3", 0x63),
    ("NUMPAD_4", 0x64),
    ("NUMPAD_5", 0x65),
    ("NUMPAD_6", 0x66),
    ("NUMPAD_7", 0x67),
    ("NUMPAD_8", 0x68),
    ("NUMPAD_9", 0x69),
    ("NUMPAD_MULTIPLY", 0x6A),
    ("NUMPAD_ADD", 0x6B),
    ("NUMPAD_SEPARATOR", 0x6C),
    ("NUMPAD_SUBTRACT", 0x6D),
    ("NUMPAD_DECIMAL", 0x6E),
    ("NUMPAD_DIVIDE", 0x6F),
    ("CTRL_L", 0xA2),
    ("CTRL_R", 0xA3),
    ("SHIFT_L", 0xA0),
    ("SHIFT_R", 0xA1),
    ("ALT_L", 0xA4),
    ("ALT_R", 0xA5),
    ("OEM_1", 0xBA),
    ("OEM_PLUS", 0xBB),
    ("OEM_COMMA", 0xBC),
    ("OEM_MINUS", 0xBD),
    ("OEM_PERIOD", 0xBE),
    ("OEM_2", 0xBF),
    ("OEM_3", 0xC0),
    ("OEM_4", 0xDB),
    ("OEM_5", 0xDC),
    ("OEM_6", 0xDD),
    ("OEM_7", 0xDE),
    ("OEM_8", 0xDF),
    ("OEM_102", 0xE2),
    ("LAUNCH_MAIL", 0xB4),
    ("MEDIA_SELECT", 0xB5),
    ("LAUNCH_APP1", 0xB6),
    ("LAUNCH_APP2", 0xB7),
    ("PLAY", 0xFA),
    ("ZOOM", 0xFB),
    ("HELP", 0xF1),
    ("ATTN", 0xF6),
    ("CRSEL", 0xF7),
    ("EXSEL", 0xF8),
    ("ERASE_EOF", 0xF9),
    ("CLEAR", 0x0C),
    ("SELECT", 0x29),
    ("EXECUTE", 0x2B),
    ("PRINT", 0x2A),
    ("OEM_CLEAR", 0xFE),
    ("PACKET", 0xE7),
    ("PROCESSKEY", 0xE5),
  ]);

  key_map
});

fn main() { 
  let version = env!("CARGO_PKG_VERSION");
  println!("github.com/nous-/disable-keys v{}", version);

  let args: Vec<String> = env::args().skip(1).collect();
  if args.is_empty() {
    eprintln!("No keys specified. Provide keys to disable (e.g., WIN_L, SLEEP, etc.). Use `--list-keys` for valid options.\n\
    Use the key names (e.g., WIN_L, SLEEP, F1) directly as arguments to disable keys.\n\
    Example: `disable-keys.exe WIN_L SLEEP F1`");
    std::process::exit(1);
  }

  let mut disabled_keys: HashSet<i32> = HashSet::new();

  if args.contains(&"--list-keys".to_string()) {
    println!("Available keys:");
    for (name, _code) in KEY_MAP.iter() {
      println!("{:}", name);
    }
    return;
  }

  for arg in args.iter() {
    let normalized_key = arg.to_uppercase();
    if let Some(&key_code) = KEY_MAP.get(normalized_key.as_str()) {
      disabled_keys.insert(key_code);
    } else {
      eprintln!("Invalid key name: {}. Use `--list-keys` for valid options.", arg);
    }
  }

  if disabled_keys.is_empty() {
    std::process::exit(1);
  }

  println!(
    "Disabled keys: {:?}\n\
    Terminate this program to re-enable.",
    args
  );

  unsafe {
    DISABLED_KEYS = Some(disabled_keys);
  }

  let hook = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), null_mut(), 0) };

  if hook.is_null() {
    panic!("Failed to set keyboard hook");
  }

  loop {
    let mut msg = winapi::um::winuser::MSG {
      hwnd: null_mut(),
      message: 0 as UINT,
      wParam: 0 as WPARAM,
      lParam: 0 as LPARAM,
      time: 0 as DWORD,
      pt: winapi::shared::windef::POINT { x: 0, y: 0 },
    };

    let get_result = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };

    if get_result == -1 {
      panic!("Failed to get message");
    }

    unsafe {
      TranslateMessage(&msg);
      DispatchMessageW(&msg);
    }

    if msg.message == winapi::um::winuser::WM_QUIT {
      break;
    }

    thread::sleep(Duration::from_millis(10));
  }

  unsafe {
    UnhookWindowsHookEx(hook);
    PostQuitMessage(0);
  }
}

unsafe extern "system" fn keyboard_hook(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  if code >= 0 {
    let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
    let vk_code = kb_struct.vkCode as i32;

    if let Some(disabled_keys) = &DISABLED_KEYS {
      if disabled_keys.contains(&vk_code) {
        if w_param == WM_KEYDOWN as WPARAM {
          return 1; // Block the keypress
        }
      }
    }
  }
  CallNextHookEx(null_mut(), code, w_param, l_param)
}