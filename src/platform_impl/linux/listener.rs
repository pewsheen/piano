use std::ptr;
use x11_dl::xlib;

pub fn run() {
  std::thread::spawn(move || {
    let xlib = xlib::Xlib::open().unwrap();
    unsafe {
      let display = (xlib.XOpenDisplay)(ptr::null());
      let root = (xlib.XDefaultRootWindow)(display);
      let mut current_focus_window: u64 = 0;

      // println!("display: {:?}, root: {:?}", display, root);

      // Only trigger key release at end of repeated keys
      #[allow(clippy::uninit_assumed_init)]
      let mut supported_rtrn: i32 = std::mem::MaybeUninit::uninit().assume_init();
      (xlib.XkbSetDetectableAutoRepeat)(display, 1, &mut supported_rtrn);

      let mut revert: i32 = 0;
      (xlib.XGetInputFocus)(display, &mut current_focus_window, &mut revert);
      (xlib.XSelectInput)(display, root, xlib::FocusChangeMask);
      #[allow(clippy::uninit_assumed_init)]
      let mut event: xlib::XEvent = std::mem::MaybeUninit::uninit().assume_init();

      // (xlib.XGrabKey)(
      //   display,
      //   xlib::AnyKey,
      //   xlib::AnyModifier,
      //   root,
      //   1,
      //   xlib::GrabModeAsync,
      //   xlib::GrabModeAsync,
      // );

      loop {
        // if (xlib.XPending)(display) > 0 {
        //   (xlib.XNextEvent)(display, &mut event);
        //   if let xlib::KeyRelease = event.get_type() {
        //     let keycode = event.key.keycode;
        //     let modifiers = event.key.state;

        //     println!("{:?}, {:?}", keycode, modifiers);
        //     xlib::XSendEvent();
        //   }
        // }
        (xlib.XNextEvent)(display, &mut event);
        if let xlib::KeyRelease = event.get_type() {
          let keycode = event.key.keycode;
          let modifiers = event.key.state;
          println!("keycode: {:?}, modifier: {:?}", keycode, modifiers);
        } else if let xlib::FocusOut = event.get_type() {
          println!("focus window: {:?}, root window: {:?}", current_focus_window, root);
          if current_focus_window != root {
            (xlib.XSelectInput)(display, current_focus_window, 0);
          }

          (xlib.XGetInputFocus)(display, &mut current_focus_window, &mut revert);
          if current_focus_window == xlib::PointerRoot as u64 {
            current_focus_window = root;
          }

          (xlib.XSelectInput)(
            display,
            current_focus_window,
            xlib::KeyReleaseMask | xlib::FocusChangeMask,
          );
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
      }
    }
  });
}
