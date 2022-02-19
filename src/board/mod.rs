use crate::ext::vec2::*;
use crate::matrix::Matrix;
use crate::tile::Tile;

mod center_view;
mod path;
mod view;

pub use center_view::CenterView;

pub struct Board {
	tiles: Matrix<Option<Tile>>,
	selected: Option<Vec2>,
	shown_path: Option<(std::time::Instant, Vec<Vec2>)>,
}

impl Board {
	pub const DEFAULT_SIZE: Vec2 = Vec2 { x: 18, y: 8 };

	const SHUFFLE_PASSES: usize = 100;

	pub fn size(&self) -> Vec2 {
		self.tiles.size()
	}

	fn tiles_unshuffled(repeats: usize) -> Vec<Option<Tile>> {
		let mut ret = Vec::with_capacity(Tile::NUM_TILES * repeats);
		for _ in 0..repeats {
			ret.extend(Tile::all().into_iter().map(Some));
		}
		ret
	}
	fn shuffle(&mut self) {
		let size = self.size();
		for _ in 0..Self::SHUFFLE_PASSES {
			let a = size.random_within();
			let b = size.random_within();

			self.tiles.swap(a, b);
		}
	}

	pub fn new(size: Vec2) -> Self {
		let total_tiles = size.area();
		assert!(total_tiles % Tile::NUM_TILES == 0);
		let mut ret = Self {
			tiles: Matrix::new(size, Self::tiles_unshuffled(total_tiles / Tile::NUM_TILES)).unwrap(),
			selected: None,
			shown_path: None,
		};
		ret.shuffle();
		ret.tiles.add_border(None);
		ret
	}
}

impl Default for Board {
	fn default() -> Self {
		Self::new(Self::DEFAULT_SIZE)
	}
}

impl Board {
	pub fn click(&mut self, pos: Vec2) {
		let tile = match self.at(pos) {
			None => return,
			Some(None) => return,
			Some(Some(tile)) => *tile,
		};
		if let Some(origin) = self.selected {
			let origin_tile = self.at(origin).unwrap().unwrap();
			if origin == pos {
				self.selected = None;
				return;
			} else if origin_tile == tile {
				if let Some(path) = self.tiles.find_path(origin, pos) {
					if let Some(origin) = self.at_mut(origin) {
						*origin = None;
					}
					if let Some(pos) = self.at_mut(pos) {
						*pos = None;
					}
					self.shown_path = Some((std::time::Instant::now(), path));
					self.selected = None;
					return;
				}
			}
		}
		self.selected = Some(pos);
	}

	pub fn at(&self, pos: Vec2) -> Option<&Option<Tile>> {
		self.tiles.get(pos)
	}
	pub fn at_mut(&mut self, pos: Vec2) -> Option<&mut Option<Tile>> {
		self.tiles.get_mut(pos)
	}
	pub fn rows(&self) -> impl Iterator<Item = &[Option<Tile>]> {
		self.tiles.rows()
	}
}
