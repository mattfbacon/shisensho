pub use cursive::Vec2;
use cursive::XY;

pub trait Vec2Ext {
	fn area(self) -> usize;
	fn width(self) -> usize;
	fn height(self) -> usize;
	fn contains(self, inner: Vec2) -> bool;
	fn max_within(self) -> Vec2;
	fn random_within(self) -> Vec2;
	fn with_x(self, new_x: usize) -> Vec2;
	fn with_y(self, new_y: usize) -> Vec2;
	fn map_x(self, f: impl FnOnce(usize) -> usize) -> Vec2;
	fn map_y(self, f: impl FnOnce(usize) -> usize) -> Vec2;
	fn move_wrapping(&mut self, movement: XY<isize>, within: Vec2);
}

impl Vec2Ext for Vec2 {
	#[inline]
	fn area(self) -> usize {
		self.x * self.y
	}
	#[inline(always)]
	fn width(self) -> usize {
		self.x
	}
	#[inline(always)]
	fn height(self) -> usize {
		self.y
	}
	fn contains(self, inner: Self) -> bool {
		inner.x < self.width() && inner.y < self.height()
	}
	#[inline]
	fn max_within(self) -> Self {
		Vec2::from((self.x - 1, self.y - 1))
	}
	fn random_within(self) -> Self {
		use rand::Rng;
		let mut rng = rand::thread_rng();
		let x = rng.gen_range(0..self.x);
		let y = rng.gen_range(0..self.y);
		Self::from((x, y))
	}
	#[inline]
	fn with_x(self, new_x: usize) -> Vec2 {
		Vec2::from((new_x, self.y))
	}
	#[inline]
	fn with_y(self, new_y: usize) -> Vec2 {
		Vec2::from((self.x, new_y))
	}
	fn map_x(self, f: impl FnOnce(usize) -> usize) -> Vec2 {
		self.with_x(f(self.x))
	}
	fn map_y(self, f: impl FnOnce(usize) -> usize) -> Vec2 {
		self.with_y(f(self.y))
	}
	fn move_wrapping(&mut self, movement: XY<isize>, within: Vec2) {
		macro_rules! do_it {
			($field:ident) => {
				if movement.$field < 0 {
					let to_sub = -movement.$field as usize;
					if let Some(new_value) = self.$field.checked_sub(to_sub) {
						self.$field = new_value;
					} else {
						self.$field = within.$field - to_sub;
					}
				} else {
					self.$field += movement.$field as usize;
					self.$field %= within.$field;
				}
			};
		}
		do_it!(x);
		do_it!(y);
	}
}
