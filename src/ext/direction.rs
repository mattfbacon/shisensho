pub use cursive::direction::Absolute as Direction;
use cursive::Vec2;

pub trait DirectionExt {
	fn joint_repr(a: Direction, b: Direction) -> &'static str;
	fn from_positions(start: Vec2, end: Vec2) -> Direction;
}

impl DirectionExt for Direction {
	fn joint_repr(a: Direction, b: Direction) -> &'static str {
		fn helper(directions: (Direction, Direction)) -> Option<&'static str> {
			Some(match directions {
				(Direction::Up, Direction::Down) => "\u{2502}",
				(Direction::Left, Direction::Right) => "\u{2500}",
				(Direction::Up, Direction::Right) => "\u{2570}",
				(Direction::Right, Direction::Down) => "\u{256d}",
				(Direction::Down, Direction::Left) => "\u{256e}",
				(Direction::Left, Direction::Up) => "\u{256f}",
				_ => return None,
			})
		}

		helper((a, b)).or_else(move || helper((b, a))).expect("No representation available for joint")
	}

	fn from_positions(start: Vec2, end: Vec2) -> Direction {
		if start == end {
			Direction::None
		} else if start.x == end.x {
			if end.y < start.y {
				Self::Up
			} else {
				Self::Down
			}
		} else if start.y == end.y {
			if end.x < start.x {
				Self::Left
			} else {
				Self::Right
			}
		} else {
			panic!("Points are not in a line ({:?} and {:?})", start, end);
		}
	}
}
