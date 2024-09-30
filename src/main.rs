use std::io::stdout;

use common::AResult;
use controller::Controller;
use crossterm::{
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};

mod application_state;
mod common;
mod controller;

struct RawMode;

impl RawMode {
  fn start() -> AResult<()> {
    terminal::enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    Ok(())
  }
}

impl Drop for RawMode {
  fn drop(&mut self) {
    terminal::disable_raw_mode();
    stdout().execute(LeaveAlternateScreen);
  }
}

fn main() -> AResult<()> {
  RawMode::start()?;

  let mut controller = Controller::new();

  controller.run();

  Ok(())
}
