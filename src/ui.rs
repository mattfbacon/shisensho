use crate::app::Frame;
use crate::game::Game;
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use tui::{
	layout::{Alignment, Rect},
	style::{Modifier, Style},
	widgets,
};

pub enum ControlFlow {
	Break(anyhow::Result<()>),
	Continue,
}

enum GameState {
	Home,
	Playing(Game),
}

impl Default for GameState {
	fn default() -> Self {
		Self::Home
	}
}

#[derive(Default)]
pub struct Ui {
	state: GameState,
}

impl Ui {
	fn update_home(&mut self, event: Event) -> ControlFlow {
		match event {
			Event::Key(event) => match event.code {
				KeyCode::Enter if event.modifiers.is_empty() => {
					self.state = GameState::Playing(Game::new());
				}
				_ => (),
			},
			_ => (),
		}
		ControlFlow::Continue
	}
	fn update_game(game: &mut Game, event: Event) -> ControlFlow {
		let (columns, rows) = match crossterm::terminal::size() {
			Ok(size) => size,
			Err(err) => return ControlFlow::Break(Err(err.into())),
		};
		let board_area = game.board_area(rows, columns);
		match event {
			Event::Mouse(event) => match event.kind {
				MouseEventKind::Up(MouseButton::Left) => {
					if board_area.intersects(Rect {
						x: event.column,
						y: event.row,
						width: 1,
						height: 1,
					}) {
						let x = (event.column - board_area.x).try_into().unwrap();
						let y = (event.row - board_area.y).try_into().unwrap();
						game.click(crate::game::board::matrix::Position::from_xy(x, y));
					}
				}
				_ => (),
			},
			_ => (),
		};
		ControlFlow::Continue
	}
	#[must_use]
	pub fn update(&mut self, event: Event) -> ControlFlow {
		match event {
			Event::Key(event) if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL => ControlFlow::Break(Ok(())),
			_ => match self.state {
				GameState::Home => self.update_home(event),
				GameState::Playing(ref mut game) => Self::update_game(game, event),
			},
		}
	}

	#[must_use]
	pub fn tick(&mut self) -> ControlFlow {
		// TODO
		ControlFlow::Continue
	}
	pub fn view(&self, term: &mut Frame) {
		match self.state {
			GameState::Home => self.view_home(term),
			GameState::Playing(ref game) => self.view_game(term, game),
		}
	}

	fn view_home(&self, term: &mut Frame) {
		let size = term.size();
		let margin = (size.height - 3) / 2;
		let width = size.width;

		let title = widgets::Paragraph::new("Shisen Sho Player").alignment(Alignment::Center).style(Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC));
		term.render_widget(title, Rect { x: 0, y: margin, width, height: 1 });

		let message = if size.width > Game::MIN_WIDTH && size.height > Game::MIN_HEIGHT {
			"Press Enter to start".to_owned()
		} else {
			format!("Terminal is not big enough. Need at least {}x{}", Game::MIN_WIDTH, Game::MIN_HEIGHT)
		};
		let action = widgets::Paragraph::new(message).alignment(Alignment::Center);
		term.render_widget(action, Rect { x: 0, y: margin + 2, width, height: 1 });
	}
	fn view_game(&self, term: &mut Frame, game: &Game) {
		term.render_widget(game, term.size());
	}
}
