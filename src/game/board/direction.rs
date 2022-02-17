use super::Position;

#[derive(Clone, Copy)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn _character_for_joint(directions: (Self, Self)) -> Option<char> {
		Some(match directions {
			(Self::Up, Self::Down) => '\u{2502}',
			(Self::Left, Self::Right) => '\u{2500}',
			(Self::Up, Self::Right) => '\u{2570}',
			(Self::Right, Self::Down) => '\u{256d}',
			(Self::Down, Self::Left) => '\u{256e}',
			(Self::Left, Self::Up) => '\u{256f}',
			_ => return None,
		})
	}
	pub fn character_for_joint(directions: (Self, Self)) -> char {
		Direction::_character_for_joint(directions).or_else(move || Direction::_character_for_joint((directions.1, directions.0))).unwrap()
	}

	pub fn from_positions(start: Position, end: Position) -> Self {
		debug_assert!(start != end);
		if start.x() == end.x() {
			if end.y() < start.y() {
				Self::Up
			} else {
				Self::Down
			}
		} else if start.y() == end.y() {
			if end.x() < start.x() {
				Self::Left
			} else {
				Self::Right
			}
		} else {
			panic!("Points are not in a line ({:?} and {:?})", start, end);
		}
	}
}
