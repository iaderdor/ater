
use crate::common::{Cursor, Mode};

pub struct ApplicationState {
  pub cursor: Cursor,
  pub mode: Mode,
}

impl ApplicationState {
  pub fn new() -> Self {
    Self {
      cursor: Cursor::new(),
      mode: Mode::Normal,
    }
  }
}
