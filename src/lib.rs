use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;

pub mod keyboard;
#[macro_use]
pub mod error;
pub mod event;
pub mod listener;
mod platform_impl;

#[macro_use]
extern crate bitflags;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

use event::KeyEvent;

static KEY_EVENT_CHANNEL: Lazy<(Sender<KeyEvent>, Receiver<KeyEvent>)> = Lazy::new(|| unbounded());

/// Gets a reference to the event channel's [Receiver<KeyEvent>]
/// which can be used to listen for menu events.
pub fn key_event_receiver<'a>() -> &'a Receiver<KeyEvent> {
  &KEY_EVENT_CHANNEL.1
}
