use std::ptr;
use x11_dl::xlib;

pub fn run() {
  std::thread::spawn(move || {
    let xlib = xlib::Xlib::open().unwrap();
    unsafe {
      let display = (xlib.XOpenDisplay)(ptr::null());
      let root = (xlib.XDefaultRootWindow)(display);
      let mut cur_window: u64 = std::mem::MaybeUninit::uninit().assume_init();
      let mut revert: i32 = std::mem::MaybeUninit::uninit().assume_init();

      (xlib.XGetInputFocus)(display, &mut cur_window, &mut revert);

      // Only trigger key release at end of repeated keys
      #[allow(clippy::uninit_assumed_init)]
      let mut supported_rtrn: i32 = std::mem::MaybeUninit::uninit().assume_init();
      (xlib.XkbSetDetectableAutoRepeat)(display, 1, &mut supported_rtrn);

      (xlib.XSelectInput)(
        display,
        cur_window,
        xlib::KeyPressMask | xlib::KeyReleaseMask | xlib::FocusChangeMask,
      );
      #[allow(clippy::uninit_assumed_init)]
      let mut event: xlib::XEvent = std::mem::MaybeUninit::uninit().assume_init();

      (xlib.XGrabKey)(
        display,
        xlib::AnyKey,
        xlib::AnyModifier,
        cur_window,
        1,
        xlib::GrabModeAsync,
        xlib::GrabModeAsync,
      );

      loop {
        if (xlib.XPending)(display) > 0 {
          (xlib.XNextEvent)(display, &mut event);
          if let xlib::KeyRelease = event.get_type() {
            let keycode = event.key.keycode;
            let modifiers = event.key.state;
            println!("keycode: {:?}, modifier: {:?}", keycode, modifiers);
            // (xlib.XSendEvent)(display, xlib::InputFocus as u64, 1, event.get_type() as i64, &mut event);
          } else if let xlib::FocusOut = event.get_type() {
            println!("cur_window: {:?}, root: {:?}", cur_window, root);
            if cur_window != root {
              (xlib.XSelectInput)(display, cur_window, 0);
              (xlib.XUngrabKey)(display, xlib::AnyKey, xlib::AnyModifier, cur_window);
            }

            (xlib.XGetInputFocus)(display, &mut cur_window, &mut revert);
            if cur_window == xlib::PointerRoot as u64 {
              cur_window = root;
            }

            (xlib.XGrabKey)(
              display,
              xlib::AnyKey,
              xlib::AnyModifier,
              cur_window,
              1,
              xlib::GrabModeAsync,
              xlib::GrabModeAsync,
            );
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
