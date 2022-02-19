use super::super::matrix::{Matrix, Position};
use super::Tile;

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
			stack: Vec<Position>,
			/// Position -> how many steps it took to reach that position (if there are fewer steps in this solution, we should not skip it)
			visited: HashMap<Position, usize>,
			goal: Position,
			goal_tile: Tile,
		}

		impl Helper<'_> {
			fn solve(&mut self, current: Position) -> Option<Vec<Position>> {
				let successors = self.matrix.successors(current, self.goal_tile);
				for &successor in successors.iter() {
					if successor == self.goal {
						self.stack.push(successor);
						return Some(self.stack.clone());
					}
				}
				if self.stack.len() < Matrix::MAX_STEPS - 1 {
					let mut best_solution: Option<Vec<Position>> = None;
					for &successor in successors.iter() {
						if self.visited.get(&successor).map(|&depth_visited| self.stack.len() >= depth_visited).unwrap_or(false) {
							continue;
						}
						self.stack.push(successor);
						self.visited.insert(successor, self.stack.len());
						if let Some(solution) = self.solve(successor) {
							if best_solution.as_ref().map(|best_solution| solution.len() < best_solution.len()).unwrap_or(true) {
								best_solution = Some(solution);
							}
						}
						self.stack.pop();
					}
					return best_solution;
				}
				None
			}
		}

		let stack = vec![start];
		let visited = HashMap::from([(start, 0)]);
		Helper {
			matrix: self,
			stack,
			visited,
			goal: end,
			goal_tile: self.get(end).unwrap().unwrap(),
		}
		.solve(start)
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

	fn check_solution(start: Position, end: Position, path: &[Position], matrix: &Matrix<Option<Tile>>) {
		assert_eq!(path[0], start);
		assert_eq!(*path.last().unwrap(), end);
		let goal_tile = matrix.get(end).unwrap().unwrap();
		for items in path.windows(2) {
			let first = items[0];
			let second = items[1];
			assert!(matrix.successors(first, goal_tile).contains(&second));
		}
	}

	#[test]
	pub fn basic() {
		let matrix = Matrix::new(Size::from_width_height(3, 3), vec![Some(Tile::Blank), None, Some(Tile::Blank), None, None, None, None, None, None]).unwrap();
		let start = Position::from_xy(0, 0);
		let end = Position::from_xy(2, 0);
		check_solution(start, end, &matrix.find_path(start, end).expect("Solution exists"), &matrix);
	}
	#[test]
	pub fn around() {
		let matrix = Matrix::new(Size::from_width_height(3, 3), vec![Some(Tile::Blank), Some(Tile::Sticks1), Some(Tile::Blank), None, None, None, None, None, None]).unwrap();
		let start = Position::from_xy(0, 0);
		let end = Position::from_xy(2, 0);
		check_solution(start, end, &matrix.find_path(start, end).expect("Solution exists"), &matrix);
	}
	#[test]
	pub fn zigzag() {
		let matrix = Matrix::new(Size::from_width_height(3, 3), vec![Some(Tile::Blank), Some(Tile::Sticks1), None, None, None, None, Some(Tile::Sticks1), Some(Tile::Blank), None]).unwrap();
		let start = Position::from_xy(0, 0);
		let end = Position::from_xy(1, 2);
		check_solution(start, end, &matrix.find_path(start, end).expect("Solution exists"), &matrix);
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
	pub fn barely_too_long() {
		let matrix = Matrix::new(Size::from_width_height(3, 3), vec![None, None, None, None, Some(Tile::Sticks1), Some(Tile::Blank), None, Some(Tile::Blank), Some(Tile::Sticks1)]).unwrap();
		assert_eq!(matrix.find_path(Position::from_xy(1, 2), Position::from_xy(2, 1)), None);
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
