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
					let current_tile = *self.get(current_pos).unwrap(); // the position is guaranteed to be inside the matrix by the caller
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
		use std::collections::{HashMap, HashSet, VecDeque};

		let mut queue = VecDeque::from([(start, 1)]);
		let mut visited = HashSet::from([start]);
		let mut traceback = HashMap::new();
		let goal_tile = self.get(end).unwrap().unwrap();

		fn trace_answer(mut current: Vec2, start: Vec2, traceback: &HashMap<Vec2, Vec2>) -> Vec<Vec2> {
			let mut ret = Vec::with_capacity(Matrix::MAX_STEPS);
			loop {
				ret.push(current);
				if current == start {
					break;
				}
				current = traceback[&current];
			}
			ret.reverse();
			ret
		}

		loop {
			let (current, steps) = match queue.pop_front() {
				Some(front) => front,
				None => break None,
			};
			if current == end {
				break Some(trace_answer(current, start, &traceback));
			}
			if steps >= Self::MAX_STEPS {
				break None;
			}
			for successor in self.successors(current, goal_tile) {
				if visited.contains(&successor) {
					continue;
				}
				traceback.insert(successor, current);
				visited.insert(successor);
				queue.push_back((successor, steps + 1));
			}
		}
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

		let matrix = Matrix::new(Vec2::new(3, 3), vec![Some(Tile::Blank), None, Some(Tile::Blank), None, None, None, None, None, None]);
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
		);
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
		let matrix = Matrix::new(Vec2::new(3, 3), vec![Some(Tile::Blank), None, Some(Tile::Blank), None, None, None, None, None, None]);
		let start = Vec2::new(0, 0);
		let end = Vec2::new(2, 0);
		check_solution(start, end, &matrix.find_path(start, end).expect("Solution exists"), 0, &matrix);
	}
	#[test]
	fn around() {
		let matrix = Matrix::new(Vec2::new(3, 3), vec![Some(Tile::Blank), Some(Tile::Sticks1), Some(Tile::Blank), None, None, None, None, None, None]);
		let start = Vec2::new(0, 0);
		let end = Vec2::new(2, 0);
		check_solution(start, end, &matrix.find_path(start, end).expect("Solution exists"), 2, &matrix);
	}
	#[test]
	fn zigzag() {
		let matrix = Matrix::new(Vec2::new(3, 3), vec![Some(Tile::Blank), Some(Tile::Sticks1), None, None, None, None, Some(Tile::Sticks1), Some(Tile::Blank), None]);
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
		);
		assert_eq!(matrix.find_path(Vec2::new(0, 0), Vec2::new(1, 2)), None);
	}
	#[test]
	pub fn barely_too_long() {
		let matrix = Matrix::new(Vec2::new(3, 3), vec![None, None, None, None, Some(Tile::Sticks1), Some(Tile::Blank), None, Some(Tile::Blank), Some(Tile::Sticks1)]);
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
		);
		assert_eq!(matrix.find_path(Vec2::new(0, 0), Vec2::new(3, 3)), None);
	}
}
