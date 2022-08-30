use std::ptr;
use x11_dl::xlib::{self, Xlib, _XDisplay};

fn grab_key(xlib: &Xlib, display: *mut _XDisplay, window: u64) {
  unsafe {
    (xlib.XGrabKey)(
      display,
      xlib::AnyKey,
      xlib::AnyModifier,
      window,
      1,
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

fn send_event(xlib: &Xlib, display: *mut _XDisplay, event: *mut xlib::XEvent) {
  unsafe {
    (xlib.XSendEvent)(display, xlib::InputFocus as u64, 1, (*event).get_type() as i64, event);
  }
}

pub fn run() {
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

      (xlib.XGetInputFocus)(display, &mut cur_window, &mut revert);
      (xlib.XSelectInput)(
        display,
        cur_window,
        xlib::KeyPressMask | xlib::KeyReleaseMask | xlib::FocusChangeMask,
      );

      grab_key(&xlib, display, cur_window);

      loop {
        if (xlib.XPending)(display) > 0 {
          (xlib.XNextEvent)(display, &mut event);
          if let xlib::KeyRelease = event.get_type() {
            let keycode = event.key.keycode;
            let modifiers = event.key.state;
            println!("keycode: {:?}, modifier: {:?}", keycode, modifiers);
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
