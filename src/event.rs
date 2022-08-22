use crate::keyboard;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyEvent {
  pub key: keyboard::KeyCode,
  pub modifiers: keyboard::ModifiersState,
  pub state: ElementState,
}

/// Describes the input state of a key.
#[non_exhaustive]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ElementState {
  Pressed,
  Released,
}
