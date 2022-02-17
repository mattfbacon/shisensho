#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
	x: usize,
	y: usize,
}

impl Position {
	pub(super) fn random(max: Position) -> Self {
		use rand::Rng;
		let mut rng = rand::thread_rng();
		Self {
			x: rng.gen_range(0..max.x),
			y: rng.gen_range(0..max.y),
		}
	}
	pub(super) fn as_matrix_idx(self) -> Option<(usize, usize)> {
		Some((self.row().try_into().ok()?, self.column().try_into().ok()?))
	}

	#[inline]
	pub const fn x(self) -> usize {
		self.x
	}
	#[inline]
	pub const fn y(self) -> usize {
		self.y
	}
	#[inline]
	pub const fn row(self) -> usize {
		self.y
	}
	#[inline]
	pub const fn column(self) -> usize {
		self.x
	}
}
