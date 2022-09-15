use std::ptr;
use x11_dl::xlib;

use crate::keyboard::KeyCode;

fn fooling(c: u8) -> u32 {
  match c {
    1 => 0,
    2 => 1,
    4 => 2,
    8 => 3,
    16 => 4,
    32 => 5,
    64 => 6,
    128 => 7,
    _ => 0,
  }
}

pub fn run() {
  std::thread::spawn(move || -> ! {
    let xlib = xlib::Xlib::open().unwrap();
    unsafe {
      let display = (xlib.XOpenDisplay)(ptr::null());

      let mut states: [bool; 32] = [false; 32];
      let mut prev_keymap: [u8; 32] = [0; 32];
      (xlib.XQueryKeymap)(display, prev_keymap.as_mut_ptr());

      loop {
        let mut keymap: [u8; 32] = [0; 32];
        (xlib.XQueryKeymap)(display, keymap.as_mut_ptr());

        for i in 0..32 {
          if prev_keymap[i] != keymap[i] {
            let keycode: u32 = i as u32 * 8 + fooling(prev_keymap[i] ^ keymap[i]);

            let physical_key = KeyCode::from_scancode(keycode as u32);

            let state: &str = match states[i] {
              true => "released",
              false => "pressed",
            };

            states[i] = !states[i];

            println!("{:?}, {:?}, {}", keycode, physical_key, state);
          }
        }

        prev_keymap = keymap;

        std::thread::sleep(std::time::Duration::from_millis(50));
      }
    }
  });
}
