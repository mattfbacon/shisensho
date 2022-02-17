use super::Board;

pub struct Game {
	started: std::time::Instant,
	board: Board,
}

impl Game {
	pub fn new() -> Self {
		Self {
			board: Board::default(),
			started: std::time::Instant::now(),
		}
	}
	pub fn board_area(&self, rows: u16, columns: u16) -> tui::layout::Rect {
		self.board
			.widget_area(tui::layout::Rect {
				x: 0,
				y: 0,
				height: rows.saturating_sub(2),
				width: columns,
			})
			.unwrap_or(tui::layout::Rect { x: 0, y: 0, height: 0, width: 0 })
	}
	pub fn click(&mut self, position: super::board::matrix::Position) {
		self.board.click(position)
	}

	pub const MIN_HEIGHT: u16 = Board::DEFAULT_SIZE.height() as u16 + 5;
	pub const MIN_WIDTH: u16 = Board::DEFAULT_SIZE.width() as u16 + 3;
}

use tui::layout::Rect;
use tui::widgets::Widget;
impl Widget for &Game {
	fn render(self, area: Rect, buf: &mut tui::buffer::Buffer) {
		let board_rect = Rect { height: area.height.saturating_sub(2), ..area };
		self.board.render(board_rect, buf);

		let elapsed = self.started.elapsed().as_secs();
		let status = tui::widgets::Paragraph::new(format!("{}:{:02} elapsed", elapsed / 60, elapsed % 60)).alignment(tui::layout::Alignment::Right);
		let status_rect = Rect {
			height: 1,
			y: area.height.saturating_sub(1),
			..area
		};
		status.render(status_rect, buf);
	}
}
