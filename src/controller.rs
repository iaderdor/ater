use std::io::{stdout, Write};

use crossterm::{
  event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
  style, QueueableCommand,
};

use crate::{
  application_state::ApplicationState,
  common::{AResult, Action, Mode},
};

pub struct Controller {
  application_state: ApplicationState,
}

impl Controller {
  pub fn new() -> Self {
    Self {
      application_state: ApplicationState::new(),
    }
  }

  pub fn run(&mut self) -> AResult<()> {
    loop {
      self.application_state.cursor.update();
      stdout().flush()?;

      let event = event::read()?;

      let action = self.handle_event(&event);

      match action {
        Some(Action::Quit) => break,
        Some(action) => self.handle_action(action),
        _ => (),
      }
    }

    Ok(())
  }

  fn handle_event(&self, event: &event::Event) -> Option<Action> {
    let mode = &self.application_state.mode;

    match mode {
      Mode::Normal => self.handle_normal_mode_event(event),
      Mode::Insert => self.handle_insert_mode_event(event),
    }
  }

  fn handle_normal_mode_event(&self, event: &event::Event) -> Option<Action> {
    match event {
      event::Event::Key(event) => match event {
        KeyEvent {
          code: KeyCode::Char('q'),
          modifiers: KeyModifiers::CONTROL,
          ..
        } => Some(Action::Quit),
        KeyEvent {
          code: KeyCode::Char('i'),
          ..
        } => Some(Action::ChangeMode(Mode::Insert)),
        KeyEvent {
          code: KeyCode::Char('j'),
          ..
        } => Some(Action::MoveLeft),
        KeyEvent {
          code: KeyCode::Char('k'),
          ..
        } => Some(Action::MoveDown),
        KeyEvent {
          code: KeyCode::Char('l'),
          ..
        } => Some(Action::MoveUp),
        KeyEvent {
          code: KeyCode::Char('ñ'),
          ..
        } => Some(Action::MoveRight),
        _ => None,
      },
      _ => None,
    }
  }

  fn handle_insert_mode_event(&self, event: &event::Event) -> Option<Action> {
    match event {
      Event::Key(event) => match event {
        KeyEvent {
          code: KeyCode::Char(c),
          ..
        } => Some(Action::InsertCharacter(*c)),
        KeyEvent {
          code: KeyCode::Esc, ..
        } => Some(Action::ChangeMode(Mode::Normal)),
        _ => None,
      },
      _ => None,
    }
  }

  fn handle_action(&mut self, action: Action) {
    match action {
      Action::ChangeMode(mode) => self.change_mode(mode),
      Action::InsertCharacter(c) => self.insert_character(c),
      Action::MoveRight => self.move_right(),
      Action::MoveLeft => self.move_left(),
      Action::MoveUp => self.move_up(),
      Action::MoveDown => self.move_down(),
      _ => (),
    }
  }

  fn insert_character(&mut self, c: char) {
    stdout().queue(style::Print(c));

    self.application_state.cursor.x += 1;
  }

  fn change_mode(&mut self, mode: Mode) {
    self.application_state.mode = mode;
  }

  fn move_right(&mut self) {
    self.application_state.cursor.x += 1;
  }

  fn move_left(&mut self) {
    self.application_state.cursor.x = self.application_state.cursor.x.saturating_sub(1);
  }

  fn move_up(&mut self) {
    self.application_state.cursor.y = self.application_state.cursor.y.saturating_sub(1);
  }

  fn move_down(&mut self) {
    self.application_state.cursor.y += 1;
  }
}
