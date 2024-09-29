use std::io::Write;

use application_state::ApplicationState;
use common::{AResult, Action, Mode};
use crossterm::{
  event::{self, KeyCode},
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand,
};

mod application_state;
mod common;

fn handle_event(application_state: &mut ApplicationState, event: &event::Event) -> Option<Action> {
  let mode = &application_state.mode;

  match mode {
    common::Mode::Normal => handle_normal_mode_event(application_state, event),
    common::Mode::Insert => todo!(),
  }
}

fn handle_normal_mode_event(
  application_state: &mut ApplicationState,
  event: &event::Event,
) -> Option<Action> {
  match event {
    event::Event::Key(event) => match event {
      event::KeyEvent {
        code: KeyCode::Char('q'),
        modifiers,
        kind,
        state,
      } => Some(Action::Quit),
      event::KeyEvent {
        code: KeyCode::Char('i'),
        modifiers,
        kind,
        state,
      } => Some(Action::ChangeMode(Mode::Insert)),
      _ => None,
    },
    _ => None,
  }
}

fn handle_action(application_state: &mut ApplicationState, action: Action) {
  if let Action::ChangeMode(mode) = action {
    change_mode(application_state, mode)
  }
}

fn change_mode(application_state: &mut ApplicationState, mode: Mode) {
  application_state.mode = mode;
}

fn main() -> AResult<()> {
  let mut application_state = ApplicationState::new();

  terminal::enable_raw_mode()?;
  application_state.stdout.execute(EnterAlternateScreen)?;
  application_state
    .stdout
    .execute(terminal::Clear(terminal::ClearType::All))?;

  loop {
    application_state
      .cursor
      .move_to(&mut application_state.stdout);
    application_state.stdout.flush()?;

    let event = event::read()?;

    let action = handle_event(&mut application_state, &event);

    match action {
      Some(Action::Quit) => break,
      Some(action) => handle_action(&mut application_state, action),
      _ => (),
    }
  }

  terminal::disable_raw_mode()?;
  application_state.stdout.execute(LeaveAlternateScreen)?;

  Ok(())
}
