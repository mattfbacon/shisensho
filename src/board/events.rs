use super::Board;
use crate::ext::vec2::*;
use cursive::event::Key;
use cursive::XY;

impl Board {
	pub fn on_click(&mut self, pos: Vec2) {
		// allow cancelling a confirmed selection by clicking on it
		if self.confirmed_selection.map(|confirmed| confirmed == pos).unwrap_or(false) {
			self.confirmed_selection = None;
		}
		// if the tile is empty, ignore the click (except for cancelling selections)
		if !self.is_occupied(pos) {
			return;
		}
		if self.confirmed_selection.is_some() {
			self.tentative_selection = Some((std::time::Instant::now(), pos));
			self.process_selections();
		} else {
			self.confirmed_selection = Some(pos);
			self.tentative_selection = None;
		}
	}
	fn process_selections(&mut self) {
		let start = self.confirmed_selection.expect("No confirmed selection");
		let end = self.tentative_selection.expect("No tentative selection").1;

		if self.at(start) != self.at(end) {
			self.confirmed_selection = Some(end);
			self.tentative_selection = None;
			return;
		}
		let path = self.tiles.find_path(start, end);
		if let Some(path) = path {
			*self.at_mut(start).expect("Confirmed selection out of range") = None;
			*self.at_mut(end).expect("Tentative selection out of range") = None;
			self.shown_path = Some((std::time::Instant::now(), path));
			self.tentative_selection = Some((std::time::Instant::now(), end));
			self.confirmed_selection = None;
		} else {
			self.confirmed_selection = Some(end);
			self.tentative_selection = None;
		}
	}
	fn confirm_selection(&mut self) {
		match (self.tentative_selection, self.confirmed_selection) {
			(Some((_, tentative)), None) => {
				if self.is_occupied(tentative) {
					self.confirmed_selection = Some(tentative);
					self.tentative_selection = None;
				}
			}
			(None, None) => (),
			(None, Some(_)) => (),
			(Some(_), Some(_)) => self.process_selections(),
		}
	}
	fn undo_selection(&mut self) {
		self.confirmed_selection = None;
	}
	fn move_selection(&mut self, x_delta: isize, y_delta: isize) {
		// TODO don't allow the selection to end up in the padding border
		if let Some(ref mut tentative_selection) = self.tentative_selection {
			tentative_selection.1.move_wrapping(XY::new(x_delta, y_delta), self.tiles.size());
			tentative_selection.0 = std::time::Instant::now();
		} else {
			self.tentative_selection = Some((std::time::Instant::now(), self.confirmed_selection.unwrap_or_else(|| Vec2::new(1, 1))));
		}
		// if the new tentative selection would overlap the confirmed selection, move it again
		if self.tentative_selection.map(|(_, sel)| sel) == self.confirmed_selection {
			let (ref mut updated, ref mut selection) = self.tentative_selection.as_mut().unwrap(); // guaranteed to be occupied as we filled it in the previous block
			selection.move_wrapping(XY::new(x_delta, y_delta), self.tiles.size());
			*updated = std::time::Instant::now();
		}
	}
	pub fn on_key(&mut self, key: Key) -> bool {
		match key {
			Key::Right => self.move_selection(1, 0),
			Key::Left => self.move_selection(-1, 0),
			Key::Up => self.move_selection(0, -1),
			Key::Down => self.move_selection(0, 1),
			Key::Enter => self.confirm_selection(),
			Key::Backspace => self.undo_selection(),
			_ => return false,
		};
		true
	}
}
