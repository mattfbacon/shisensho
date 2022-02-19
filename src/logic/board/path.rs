use super::super::matrix::{Matrix, Position};
use super::Tile;
use std::collections::VecDeque;

impl Matrix<Option<Tile>> {
	const MAX_STEPS: usize = 2 + 2; // 2 turns + start + end
	fn successors(&self, pos: Position, goal_tile: Tile) -> Vec<Position> {
		let mut ret = Vec::new();
		let size = self.size();

		let with_y = move |y| pos.with_y(y);
		let with_x = move |x| pos.with_x(x);
		macro_rules! add_items_for {
			($iter:expr) => {
				for current_pos in $iter {
					let current_tile = *self.get(current_pos).unwrap();
					if current_tile.is_none() {
						ret.push(current_pos);
						continue;
					} else if current_tile == Some(goal_tile) {
						ret.push(current_pos);
						break;
					} else {
						break;
					}
				}
			};
		}
		// left
		add_items_for!((0..pos.x()).into_iter().rev().map(with_x));
		// right
		add_items_for!(((pos.x() + 1)..size.width()).into_iter().map(with_x));
		// above
		add_items_for!((0..pos.y()).into_iter().rev().map(with_y));
		// below
		add_items_for!((pos.y() + 1..size.height()).into_iter().map(with_y));

		ret
	}
	/// Returns the corners of the path including the start and end, if a path could be found.
	pub(super) fn find_path(&self, start: Position, end: Position) -> Option<Vec<Position>> {
		use std::collections::HashMap;

		struct Helper<'a> {
			matrix: &'a Matrix<Option<Tile>>,
			queue: VecDeque<(Position, i32)>,
			traceback: HashMap<Position, Position>,
			visited: HashMap<Position, usize>,
			goal: Position,
			goal_tile: Tile,
			start: Position,
			maximum_steps: usize,
		}

		impl Helper<'_> {
			fn solve(&mut self) -> Option<Vec<Position>> {
				fn trace_answer(goal: Position, start: Position, traceback: &mut HashMap<Position, Position>) -> Vec<Position> {
					let mut current = goal;
					let mut ret = Vec::new();
					loop {
						ret.push(current);
						if current == start {
							break;
						}
						current = traceback[&current];
					}
					// reverse the matrix
					ret.reverse();
					ret
				}
				while self.queue.len() > 0 {
					let (current, steps) = self.queue.pop_front().unwrap();
					if current == self.goal {
						return Some(trace_answer(self.goal, self.start, &mut self.traceback));
					}
					if steps > self.maximum_steps.try_into().unwrap() {
						break;
					}
					for successor in self.matrix.successors(current, self.goal_tile) {
						if self.visited.contains_key(&successor) {
							continue;
						}
						self.traceback.insert(successor, current);
						self.visited.insert(successor, 1);
						self.queue.push_back((successor, steps + 1));
					}
				}
				None
			}
		}

		let mut queue = VecDeque::new();
		queue.push_back((start, 0));
		let visited = HashMap::from([(start, 1)]);
		Helper {
			matrix: self,
			queue,
			traceback: HashMap::new(),
			visited,
			goal: end,
			goal_tile: self.get(end).unwrap().unwrap(),
			start,
			maximum_steps: Self::MAX_STEPS,
		}
		.solve()
	}
}

#[cfg(test)]
mod test {
	use super::super::{Matrix, Position, Size, Tile};

	#[test]
	pub fn successors_easy() {
		use std::collections::HashSet;

		let matrix = Matrix::new(Size::from_width_height(3, 3), vec![Some(Tile::Blank), None, Some(Tile::Blank), None, None, None, None, None, None]).unwrap();
		assert_eq!(
			matrix.successors(Position::from_xy(0, 0), Tile::Blank).into_iter().collect::<HashSet<_>>(),
			HashSet::from([Position::from_xy(0, 1), Position::from_xy(0, 2), Position::from_xy(1, 0), Position::from_xy(2, 0)])
		);
	}
	#[test]
	pub fn successors_hard() {
		use std::collections::HashSet;

		let matrix = Matrix::new(
			Size::from_width_height(4, 4),
			vec![
				Some(Tile::Blank),
				Some(Tile::Sticks1),
				Some(Tile::Blank),
				None,
				None,
				None,
				None,
				None,
				None,
				Some(Tile::Sticks1),
				None,
				None,
				None,
				None,
				Some(Tile::Blank),
				None,
			],
		)
		.unwrap();
		assert_eq!(
			matrix.successors(Position::from_xy(2, 0), Tile::Blank).into_iter().collect::<HashSet<_>>(),
			HashSet::from([Position::from_xy(3, 0), Position::from_xy(2, 1), Position::from_xy(2, 2), Position::from_xy(2, 3)])
		);
	}
	#[test]
	pub fn basic() {
		let matrix = Matrix::new(Size::from_width_height(3, 3), vec![Some(Tile::Blank), None, Some(Tile::Blank), None, None, None, None, None, None]).unwrap();
		assert_eq!(
			matrix.find_path(Position::from_xy(0, 0), Position::from_xy(2, 0)).expect("Solution exists").as_slice(),
			&[Position::from_xy(0, 0), Position::from_xy(2, 0)]
		);
	}
	#[test]
	pub fn around() {
		let matrix = Matrix::new(Size::from_width_height(3, 3), vec![Some(Tile::Blank), Some(Tile::Sticks1), Some(Tile::Blank), None, None, None, None, None, None]).unwrap();
		assert_eq!(
			matrix.find_path(Position::from_xy(0, 0), Position::from_xy(2, 0)).expect("Solution exists").as_slice(),
			&[Position::from_xy(0, 0), Position::from_xy(0, 1), Position::from_xy(2, 1), Position::from_xy(2, 0)]
		);
	}
	#[test]
	pub fn zigzag() {
		let matrix = Matrix::new(Size::from_width_height(3, 3), vec![Some(Tile::Blank), Some(Tile::Sticks1), None, None, None, None, Some(Tile::Sticks1), Some(Tile::Blank), None]).unwrap();
		assert_eq!(
			matrix.find_path(Position::from_xy(0, 0), Position::from_xy(1, 2)).expect("Solution exists").as_slice(),
			&[Position::from_xy(0, 0), Position::from_xy(0, 1), Position::from_xy(1, 1), Position::from_xy(1, 2)]
		);
	}
	#[test]
	pub fn no_path() {
		let matrix = Matrix::new(
			Size::from_width_height(3, 3),
			vec![
				Some(Tile::Blank),
				Some(Tile::Sticks1),
				Some(Tile::Sticks1),
				Some(Tile::Sticks1),
				Some(Tile::Sticks1),
				Some(Tile::Sticks1),
				Some(Tile::Sticks1),
				Some(Tile::Blank),
				Some(Tile::Sticks1),
			],
		)
		.unwrap();
		assert_eq!(matrix.find_path(Position::from_xy(0, 0), Position::from_xy(1, 2)), None);
	}
	#[test]
	pub fn too_long() {
		let matrix = Matrix::new(
			Size::from_width_height(4, 4),
			vec![
				Some(Tile::Blank),
				Some(Tile::Sticks1),
				None,
				None,
				None,
				None,
				Some(Tile::Sticks1),
				None,
				Some(Tile::Sticks1),
				None,
				None,
				Some(Tile::Sticks1),
				None,
				Some(Tile::Sticks1),
				None,
				Some(Tile::Blank),
			],
		)
		.unwrap();
		assert_eq!(matrix.find_path(Position::from_xy(0, 0), Position::from_xy(3, 3)), None);
	}
}
