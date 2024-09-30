use std::io::Write;

use application_state::ApplicationState;
use common::{AResult, Action, Mode};
use crossterm::{
  event::{self, KeyCode, KeyModifiers},
  style,
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  ExecutableCommand, QueueableCommand,
};

mod application_state;
mod common;

fn handle_event(application_state: &mut ApplicationState, event: &event::Event) -> Option<Action> {
  let mode = &application_state.mode;

  match mode {
    common::Mode::Normal => handle_normal_mode_event(application_state, event),
    common::Mode::Insert => handle_insert_mode_event(event),
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
        modifiers: KeyModifiers::CONTROL,
        ..
      } => Some(Action::Quit),
      event::KeyEvent {
        code: KeyCode::Char('i'),
        ..
      } => Some(Action::ChangeMode(Mode::Insert)),
      _ => None,
    },
    _ => None,
  }
}

fn handle_insert_mode_event(event: &event::Event) -> Option<Action> {
  match event {
    event::Event::Key(event) => match event {
      event::KeyEvent {
        code: KeyCode::Char(c),
        ..
      } => Some(Action::InsertCharacter(*c)),
      event::KeyEvent {
        code: KeyCode::Esc, ..
      } => Some(Action::ChangeMode(Mode::Normal)),
      _ => None,
    },
    _ => None,
  }
}

fn handle_action(application_state: &mut ApplicationState, action: Action) {
  match action {
    Action::ChangeMode(mode) => change_mode(application_state, mode),
    Action::InsertCharacter(c) => insert_character(application_state, c),
    _ => (),
  }
}

fn insert_character(application_state: &mut ApplicationState, c: char) {
  application_state.stdout.queue(style::Print(c));

  application_state.cursor.x += 1;
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
