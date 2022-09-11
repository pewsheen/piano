mod keycode;
mod keyboard;
pub mod listener;

pub use self::keycode::{keycode_from_scancode, keycode_to_scancode};

#[derive(Debug, Clone)]
pub struct OsError;

impl std::fmt::Display for OsError {
  fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    Ok(())
  }
}
