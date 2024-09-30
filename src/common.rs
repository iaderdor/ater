use std::io::stdout;

use crossterm::{cursor, QueueableCommand};

pub type AResult<T> = anyhow::Result<T>;

pub enum Mode {
  Normal,
  Insert,
}

pub enum Action {
  Quit,

  ChangeMode(Mode),

  MoveRight,
  MoveLeft,
  MoveUp,
  MoveDown,

  InsertCharacter(char),
}

pub struct Cursor {
  pub x: u16,
  pub y: u16,
}

impl Cursor {
  pub fn new() -> Self {
    Self { x: 0, y: 0 }
  }

  pub fn update(&self) {
    stdout().queue(cursor::MoveTo(self.x, self.y));
  }
}
