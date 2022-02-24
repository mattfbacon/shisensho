use crate::ext::vec2::*;
use crate::matrix::Matrix;
use crate::tile::Tile;

mod center_view;
mod events;
mod path;
mod view;

pub use center_view::CenterView;

pub struct Board {
	tiles: Matrix<Option<Tile>>,
	confirmed_selection: Option<Vec2>,
	// the Instant stores when the selection was last updated and is used for blinking
	tentative_selection: Option<(std::time::Instant, Vec2)>,
	// the Instant stores when the match with the shown path was made and is used for fading
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
			tiles: Matrix::new(size, Self::tiles_unshuffled(total_tiles / Tile::NUM_TILES)),
			confirmed_selection: None,
			tentative_selection: None,
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
	pub fn at(&self, pos: Vec2) -> Option<Tile> {
		match self.tiles.get(pos) {
			None => None,
			Some(&maybe_tile) => maybe_tile,
		}
	}
	pub fn at_mut(&mut self, pos: Vec2) -> Option<&mut Option<Tile>> {
		self.tiles.get_mut(pos)
	}
	pub fn is_occupied(&self, pos: Vec2) -> bool {
		self.at(pos).is_some()
	}
	pub fn rows(&self) -> impl Iterator<Item = &[Option<Tile>]> {
		self.tiles.rows()
	}
}
