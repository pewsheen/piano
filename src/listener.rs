use crate::platform_impl::listener::run;

pub fn run_listener(window: &gtk::ApplicationWindow) {
  run(window);
}
