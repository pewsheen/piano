use gtk::{gdk::EventKey, prelude::Continue, traits::WidgetExt, Inhibit};
use std::{ptr, rc::Rc};
use x11_dl::xlib::{self, Xlib, _XDisplay};

use crate::{
  event::ElementState,
  keyboard::{self, KeyCode, ModifiersState},
};

use super::keyboard::get_modifiers;

fn grab_key(xlib: &Xlib, display: *mut _XDisplay, window: u64) {
  unsafe {
    (xlib.XGrabKey)(
      display,
      xlib::AnyKey,
      xlib::AnyModifier,
      window,
      0,
      xlib::GrabModeAsync,
      xlib::GrabModeAsync,
    );
  }
}

fn ungrab_key(xlib: &Xlib, display: *mut _XDisplay, window: u64) {
  unsafe {
    (xlib.XUngrabKey)(display, xlib::AnyKey, xlib::AnyModifier, window);
  }
}

fn send_event(xlib: &Xlib, display: *mut _XDisplay, window: u64, event: *mut xlib::XEvent) {
  unsafe {
    (xlib.XSendEvent)(
      display,
      window,
      1,
      (*event).get_type() as i64,
      event,
    );
  }
}

pub fn run(window: &gtk::ApplicationWindow) {
  let keyboard_handler = Rc::new(move |event_key: EventKey, element_state| {
    // if we have a modifier lets send it
    let mut mods = get_modifiers(event_key.clone());
    if !mods.is_empty() {
      // if we release the modifier tell the world
      if ElementState::Released == element_state {
        mods = ModifiersState::empty();
      }
    }

    let scancode = event_key.hardware_keycode();
    let physical_key = KeyCode::from_scancode(scancode as u32);

    // todo: implement repeat?
    // let event = make_key_event(&event_key, false, None, element_state);

    println!("key: {:?}, modifier: {:?}", physical_key, mods);

    Continue(true)
  });

  let handler = keyboard_handler.clone();
  window.connect_key_press_event(move |_, event_key| {
    handler(event_key.to_owned(), ElementState::Pressed);
    // ime.filter_keypress(event_key);
    println!("{:?}", event_key);

    Inhibit(true)
  });

  std::thread::spawn(move || {
    let xlib = xlib::Xlib::open().unwrap();
    unsafe {
      let display = (xlib.XOpenDisplay)(ptr::null());
      let root = (xlib.XDefaultRootWindow)(display);
      let mut cur_window: u64 = std::mem::MaybeUninit::uninit().assume_init();
      let mut revert: i32 = std::mem::MaybeUninit::uninit().assume_init();

      // Only trigger key release at end of repeated keys
      #[allow(clippy::uninit_assumed_init)]
      let mut supported_rtrn: i32 = std::mem::MaybeUninit::uninit().assume_init();
      (xlib.XkbSetDetectableAutoRepeat)(display, 1, &mut supported_rtrn);

      #[allow(clippy::uninit_assumed_init)]
      let mut event: xlib::XEvent = std::mem::MaybeUninit::uninit().assume_init();

      // (xlib.XSelectInput)(display, root, xlib::KeyPressMask | xlib::KeyReleaseMask);

      grab_key(&xlib, display, root);

      loop {
        if (xlib.XPending)(display) > 0 {
          (xlib.XNextEvent)(display, &mut event);
          if let xlib::KeyRelease = event.get_type() {
            let keycode = event.key.keycode;
            let modifiers = event.key.state;
            println!("keycode: {:?}, modifier: {:?}", keycode, modifiers);
            send_event(&xlib, display, root, &mut event);
          } else if let xlib::FocusOut = event.get_type() {
            println!("prev: {:?}", cur_window);
            // unregister focus out window's event mask.
            if cur_window != root {
              (xlib.XSelectInput)(display, cur_window, 0);
              ungrab_key(&xlib, display, cur_window);
            }

            // get current focused window.
            (xlib.XGetInputFocus)(display, &mut cur_window, &mut revert);
            if cur_window == xlib::PointerRoot as u64 {
              cur_window = root;
            }
            println!("curr: {:?}", cur_window);

            // register current focused window's event mask.
            grab_key(&xlib, display, cur_window);
            (xlib.XSelectInput)(
              display,
              root,
              xlib::KeyPressMask | xlib::KeyReleaseMask | xlib::FocusChangeMask,
            );
          }
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
      }
    }
  });
}
