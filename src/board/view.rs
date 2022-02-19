use super::Board;
use crate::ext::{direction::*, vec2::*};
use cursive::event::{Event, EventResult};
use cursive::{theme::Effect, Printer, Rect, View};

/*
impl Board {
	pub fn widget_area(&self, rect: tui::layout::Rect) -> Option<tui::layout::Rect> {
		let size = self.size();
		let board_height: u16 = size.height().try_into().unwrap();
		let board_width: u16 = size.width().try_into().unwrap();

		if rect.height < board_height || rect.width < board_width {
			None
		} else {
			let left = (rect.width - board_width) / 2 + rect.x;
			let top = (rect.height - board_height) / 2 + rect.y;
			Some(Rect {
				x: left,
				y: top,
				height: board_height,
				width: board_width,
			})
		}
	}
}
*/

impl View for Board {
	fn draw(&self, printer: &Printer<'_, '_>) {
		for (y, row) in self.rows().enumerate() {
			for (x, tile) in row.iter().enumerate() {
				let pos = Vec2::from((x, y));
				printer.with_effect(if self.selected == Some(pos.into()) { Effect::Reverse } else { Effect::Simple }, move |printer| {
					printer.print(pos, tile.map(|tile| tile.repr()).unwrap_or(" "));
				})
			}
		}

		if let Some((ref shown_time, ref path)) = self.shown_path {
			let shown_time = shown_time.elapsed().as_millis();
			if shown_time < 400 {
				let effect = if shown_time < 200 { Effect::Simple } else { Effect::Dim };
				printer.with_effect(effect, move |printer| {
					for lines in path.windows(2) {
						let start = lines[0];
						let end = lines[1];
						if start.x == end.x {
							// vertical line
							let x = start.x;
							let (start, end) = if start.y < end.y { (start.y, end.y) } else { (end.y, start.y) };
							let (start, end) = (start + 1, end - 1);
							for y in start..=end {
								let pos = Vec2::from((x, y));
								printer.print(pos, "\u{2502}");
							}
						} else if start.y == end.y {
							// horizontal line
							let y = start.y;
							let (start, end) = if start.x < end.x { (start.x, end.x) } else { (end.x, start.x) };
							let (start, end) = (start + 1, end - 1);
							for x in start..=end {
								let pos = Vec2::from((x, y));
								printer.print(pos, "\u{2500}");
							}
						} else {
							panic!("Path line is neither vertical nor horizontal ({:?} to {:?})", start, end);
						}
					}
					for corners in path.windows(3) {
						let corner_pos = corners[1];
						let joint = Direction::joint_repr(Direction::from_positions(corner_pos, corners[0]), Direction::from_positions(corner_pos, corners[2]));
						printer.print(corner_pos, joint);
					}
				});
			}
		}
	}
	fn needs_relayout(&self) -> bool {
		false
	}
	fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
		self.tiles.size()
	}
	fn on_event(&mut self, event: Event) -> EventResult {
		use cursive::event::{MouseButton, MouseEvent};
		match event {
			Event::FocusLost => {
				self.selected = None;
				EventResult::Consumed(None)
			}
			Event::Mouse {
				offset,
				position,
				event: MouseEvent::Release(MouseButton::Left),
			} => {
				if Rect::from_size(offset, self.tiles.size()).contains(position) {
					self.click(position - offset);
					EventResult::Consumed(None)
				} else {
					EventResult::Ignored
				}
			}
			_ => EventResult::Ignored,
		}
	}
	/*
	fn render(self, area: Rect, buf: &mut Buffer) {
		let area = if let Some(area) = self.widget_area(area) {
			area
		} else {
			Paragraph::new("Too small").style(Style::default().fg(Color::Red)).render(area, buf);
			return;
		};

		let left = area.x;
		let top = area.y;

		for (row_num, row) in self.rows().enumerate() {
			let top = top + u16::try_from(row_num).unwrap();
			for (col_num, tile) in row.iter().enumerate() {
				let left = left + u16::try_from(col_num).unwrap();
				buf.get_mut(left, top).reset();
				buf.get_mut(left, top).set_char(tile.as_ref().map(|tile| tile.as_char()).unwrap_or(' '));
				if self.selected == Some(Position::from_row_col(row_num, col_num)) {
					buf.get_mut(left, top).set_style(Style::default().add_modifier(Modifier::REVERSED));
				}
			}
		}

		if let Some((ref shown_time, ref path)) = self.shown_path {
			let shown_time = shown_time.elapsed().as_millis();
			if shown_time < 400 {
				let style = if shown_time < 200 { Style::default() } else { Style::default().add_modifier(Modifier::DIM) };
				for lines in path.windows(2) {
					let start = lines[0];
					let end = lines[1];
					if start.x() == end.x() {
						// vertical line
						let x = start.x();
						let (start, end) = if start.y() < end.y() { (start.y(), end.y()) } else { (end.y(), start.y()) };
						let (start, end) = (start + 1, end - 1);
						for y in start..=end {
							// left and top are defined two blocks before
							let cell = buf.get_mut(left + u16::try_from(x).unwrap(), top + u16::try_from(y).unwrap());
							cell.reset();
							cell.set_char('\u{2502}');
							cell.set_style(style);
						}
					} else if start.y() == end.y() {
						// horizontal line
						let y = start.y();
						let (start, end) = if start.x() < end.x() { (start.x(), end.x()) } else { (end.x(), start.x()) };
						let (start, end) = (start + 1, end - 1);
						for x in start..=end {
							// left and top are defined two blocks before
							let cell = buf.get_mut(left + u16::try_from(x).unwrap(), top + u16::try_from(y).unwrap());
							cell.reset();
							cell.set_char('\u{2500}');
							cell.set_style(style);
						}
					} else {
						panic!("Path line is neither vertical nor horizontal ({:?} to {:?})", start, end);
					}
				}
				for corners in path.windows(3) {
					let corner_pos = corners[1];
					let joint = Direction::character_for_joint((Direction::from_positions(corner_pos, corners[0]), Direction::from_positions(corner_pos, corners[2])));
					let cell = buf.get_mut(left + u16::try_from(corner_pos.x()).unwrap(), top + u16::try_from(corner_pos.y()).unwrap());
					cell.reset();
					cell.set_char(joint);
					cell.set_style(style);
				}
			}
		}
	}
	*/
}
