use crate::ext::vec2::*;

pub struct Matrix<T> {
	size: Vec2,
	data: Vec<T>,
}

impl<T> Matrix<T> {
	pub fn new(size: Vec2, data: Vec<T>) -> Option<Self> {
		if size.area() != data.len() {
			None
		} else {
			Some(Self { size, data })
		}
	}
	pub fn size(&self) -> Vec2 {
		self.size
	}
	pub fn rows(&self) -> impl Iterator<Item = &[T]> {
		self.data.chunks_exact(self.size.width())
	}

	fn index(&self, position: Vec2) -> Option<usize> {
		if self.size.contains(position) {
			Some((position.y * self.size.width()) + position.x)
		} else {
			None
		}
	}
	pub fn swap(&mut self, a: Vec2, b: Vec2) {
		let a = self.index(a).unwrap();
		let b = self.index(b).unwrap();
		self.data.swap(a, b);
	}
	pub fn get(&self, position: Vec2) -> Option<&T> {
		self.data.get(self.index(position)?)
	}
	pub fn get_mut(&mut self, position: Vec2) -> Option<&mut T> {
		let index = self.index(position)?;
		self.data.get_mut(index)
	}

	/// Adds a 1-element-wide border of the given element around the matrix on all sides
	pub fn add_border(&mut self, element: T)
	where
		T: Copy, // could be Clone if we didn't use `self.rows()` but that's unnecessary and overcomplicated
	{
		use std::iter::repeat;

		self.data = {
			let mut data = Vec::with_capacity(self.data.capacity() + (self.size().width() * 2) + (self.size().height() * 2) + 4);
			data.extend(repeat(element).take(self.size.width() + 2));
			for row in self.rows() {
				data.push(element);
				data.extend(row.iter().copied());
				data.push(element);
			}
			data.extend(repeat(element).take(self.size.width() + 2));
			data
		};
		self.size.x += 2;
		self.size.y += 2;
	}
}
