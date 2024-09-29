use std::io::stdout;

use crate::common::{Cursor, Mode};

pub struct ApplicationState {
  pub cursor: Cursor,
  pub stdout: std::io::Stdout,
  pub mode: Mode,
}

impl ApplicationState {
  pub fn new() -> Self {
    Self {
      cursor: Cursor::new(),
      stdout: stdout(),
      mode: Mode::Normal,
    }
  }
}
