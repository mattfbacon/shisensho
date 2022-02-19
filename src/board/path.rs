use super::Tile;
use crate::ext::vec2::*;
use crate::matrix::Matrix;

impl Matrix<Option<Tile>> {
	const MAX_STEPS: usize = 2 + 2; // 2 turns + start + end

	fn successors(&self, pos: Vec2, goal_tile: Tile) -> Vec<Vec2> {
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
		add_items_for!((0..pos.x).into_iter().rev().map(with_x));
		// right
		add_items_for!(((pos.x + 1)..size.width()).into_iter().map(with_x));
		// above
		add_items_for!((0..pos.y).into_iter().rev().map(with_y));
		// below
		add_items_for!((pos.y + 1..size.height()).into_iter().map(with_y));

		ret
	}
	/// Returns the corners of the path including the start and end, if a path could be found.
	pub(super) fn find_path(&self, start: Vec2, end: Vec2) -> Option<Vec<Vec2>> {
		use std::collections::HashMap;

		struct Helper<'a> {
			matrix: &'a Matrix<Option<Tile>>,
			stack: Vec<Vec2>,
			/// Position -> how many steps it took to reach that position (if there are fewer steps in this solution, we should not skip it)
			visited: HashMap<Vec2, usize>,
			goal: Vec2,
			goal_tile: Tile,
		}

		impl Helper<'_> {
			fn solve(&mut self, current: Vec2) -> Option<Vec<Vec2>> {
				let successors = self.matrix.successors(current, self.goal_tile);
				for &successor in successors.iter() {
					if successor == self.goal {
						self.stack.push(successor);
						return Some(self.stack.clone());
					}
				}
				if self.stack.len() < Matrix::MAX_STEPS - 1 {
					let mut best_solution: Option<Vec<Vec2>> = None;
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
	use crate::matrix::Matrix;
	use crate::tile::Tile;
	use cursive::Vec2;

	#[test]
	fn successors_easy() {
		use std::collections::HashSet;

		let matrix = Matrix::new(Vec2::new(3, 3), vec![Some(Tile::Blank), None, Some(Tile::Blank), None, None, None, None, None, None]).unwrap();
		assert_eq!(
			matrix.successors(Vec2::new(0, 0), Tile::Blank).into_iter().collect::<HashSet<_>>(),
			HashSet::from([Vec2::new(0, 1), Vec2::new(0, 2), Vec2::new(1, 0), Vec2::new(2, 0)])
		);
	}
	#[test]
	fn successors_hard() {
		use std::collections::HashSet;

		let matrix = Matrix::new(
			Vec2::new(4, 4),
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
			matrix.successors(Vec2::new(2, 0), Tile::Blank).into_iter().collect::<HashSet<_>>(),
			HashSet::from([Vec2::new(3, 0), Vec2::new(2, 1), Vec2::new(2, 2), Vec2::new(2, 3)])
		);
	}

	fn check_solution(start: Vec2, end: Vec2, path: &[Vec2], expected_turns: usize, matrix: &Matrix<Option<Tile>>) {
		assert_eq!(path[0], start);
		assert_eq!(*path.last().unwrap(), end);
		assert_eq!(path.len(), expected_turns + 2);
		let goal_tile = matrix.get(end).unwrap().unwrap();
		for items in path.windows(2) {
			let first = items[0];
			let second = items[1];
			assert!(matrix.successors(first, goal_tile).contains(&second));
		}
	}

	#[test]
	fn basic() {
		let matrix = Matrix::new(Vec2::new(3, 3), vec![Some(Tile::Blank), None, Some(Tile::Blank), None, None, None, None, None, None]).unwrap();
		let start = Vec2::new(0, 0);
		let end = Vec2::new(2, 0);
		check_solution(start, end, &matrix.find_path(start, end).expect("Solution exists"), 0, &matrix);
	}
	#[test]
	fn around() {
		let matrix = Matrix::new(Vec2::new(3, 3), vec![Some(Tile::Blank), Some(Tile::Sticks1), Some(Tile::Blank), None, None, None, None, None, None]).unwrap();
		let start = Vec2::new(0, 0);
		let end = Vec2::new(2, 0);
		check_solution(start, end, &matrix.find_path(start, end).expect("Solution exists"), 2, &matrix);
	}
	#[test]
	fn zigzag() {
		let matrix = Matrix::new(Vec2::new(3, 3), vec![Some(Tile::Blank), Some(Tile::Sticks1), None, None, None, None, Some(Tile::Sticks1), Some(Tile::Blank), None]).unwrap();
		let start = Vec2::new(0, 0);
		let end = Vec2::new(1, 2);
		check_solution(start, end, &matrix.find_path(start, end).expect("Solution exists"), 2, &matrix);
	}
	#[test]
	fn no_path() {
		let matrix = Matrix::new(
			Vec2::new(3, 3),
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
		assert_eq!(matrix.find_path(Vec2::new(0, 0), Vec2::new(1, 2)), None);
	}
	#[test]
	pub fn barely_too_long() {
		let matrix = Matrix::new(Vec2::new(3, 3), vec![None, None, None, None, Some(Tile::Sticks1), Some(Tile::Blank), None, Some(Tile::Blank), Some(Tile::Sticks1)]).unwrap();
		assert_eq!(matrix.find_path(Vec2::new(1, 2), Vec2::new(2, 1)), None);
	}
	#[test]
	fn too_long() {
		let matrix = Matrix::new(
			Vec2::new(4, 4),
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
		assert_eq!(matrix.find_path(Vec2::new(0, 0), Vec2::new(3, 3)), None);
	}
}
