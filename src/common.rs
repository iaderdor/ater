use std::io::Stdout;

use crossterm::{cursor, QueueableCommand};

pub type AResult<T> = anyhow::Result<T>;

pub enum Mode {
  Normal,
  Insert,
}

pub enum Action {
  Quit,

  ChangeMode(Mode),
}

pub struct Cursor {
  pub x: u16,
  pub y: u16,
}

impl Cursor {
  pub fn new() -> Self {
    Self { x: 0, y: 0 }
  }

  pub fn move_to(&self, stdout: &mut Stdout) {
    stdout.queue(cursor::MoveTo(self.x, self.y));
  }
}
