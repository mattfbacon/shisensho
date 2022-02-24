use super::Board;
use crate::ext::{direction::*, vec2::*};
use cursive::event::{Event, EventResult};
use cursive::{theme::Effect, Printer, Rect, View};

impl View for Board {
	fn draw(&self, printer: &Printer<'_, '_>) {
		for (y, row) in self.rows().enumerate() {
			for (x, tile) in row.iter().enumerate() {
				let pos = Vec2::from((x, y));
				let style = if self.confirmed_selection.map(|sel| sel == pos).unwrap_or(false) {
					Effect::Reverse
				} else if self.tentative_selection.map(|(_, sel)| sel == pos).unwrap_or(false) {
					// the if condition would return false if the tentative selection was None. This is admittedly a bit ugly but I couldn't think of a better way.
					let blink_on = self.tentative_selection.unwrap().0.elapsed().subsec_millis() < 500;
					if blink_on {
						Effect::Reverse
					} else {
						Effect::Simple
					}
				} else {
					Effect::Simple
				};
				printer.with_effect(style, move |printer| {
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
			Event::Mouse {
				offset,
				position,
				event: MouseEvent::Release(MouseButton::Left),
			} => {
				if Rect::from_size(offset, self.tiles.size()).contains(position) {
					self.on_click(position - offset);
					EventResult::Consumed(None)
				} else {
					EventResult::Ignored
				}
			}
			Event::Key(key) => {
				if self.on_key(key) {
					EventResult::Consumed(None)
				} else {
					EventResult::Ignored
				}
			}
			_ => EventResult::Ignored,
		}
	}
}
