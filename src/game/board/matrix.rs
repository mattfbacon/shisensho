pub struct Matrix<T> {
	size: Size,
	data: Vec<T>,
}

#[derive(Clone, Copy)]
pub struct Size {
	width: usize,
	height: usize,
}
impl Size {
	#[inline]
	pub const fn width(self) -> usize {
		self.width
	}
	#[inline]
	pub const fn height(self) -> usize {
		self.height
	}
	#[inline]
	pub const fn rows(self) -> usize {
		self.height
	}
	#[inline]
	pub const fn columns(self) -> usize {
		self.width
	}
	#[inline]
	pub const fn area(self) -> usize {
		self.width * self.height
	}

	pub const fn from_width_height(width: usize, height: usize) -> Self {
		Self { width, height }
	}
	pub const fn from_rows_cols(rows: usize, columns: usize) -> Self {
		Self { width: columns, height: rows }
	}

	pub const fn contains(self, point: Position) -> bool {
		point.x() < self.width() && point.y() < self.height()
	}
	pub const fn max_within(self) -> Position {
		Position::from_row_col(self.rows() - 1, self.columns() - 1)
	}
	pub fn random_within(self) -> Position {
		use rand::Rng;
		let mut rng = rand::thread_rng();
		let row = rng.gen_range(0..self.rows());
		let col = rng.gen_range(0..self.columns());
		Position::from_row_col(row, col)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
	x: usize,
	y: usize,
}
impl Position {
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

	pub const fn from_xy(x: usize, y: usize) -> Self {
		Self { x, y }
	}
	pub const fn from_row_col(row: usize, column: usize) -> Self {
		Self { x: column, y: row }
	}

	pub const fn with_x(self, x: usize) -> Self {
		Self { x, ..self }
	}
	pub const fn with_y(self, y: usize) -> Self {
		Self { y, ..self }
	}
	pub const fn with_row(self, row: usize) -> Self {
		Self::from_row_col(row, self.column())
	}
	pub const fn with_column(self, column: usize) -> Self {
		Self::from_row_col(self.row(), column)
	}

	pub const fn is_neighbor_with(self, other: Position) -> bool {
		let horizontal_neighbor = self.x + 1 == other.x || other.x + 1 == self.x;
		let vertical_neighbor = self.y + 1 == other.y || other.y + 1 == self.y;
		horizontal_neighbor ^ vertical_neighbor
	}
}

impl<T> Matrix<T> {
	pub fn new(size: Size, data: Vec<T>) -> Option<Self> {
		if size.area() != data.len() {
			None
		} else {
			Some(Self { size, data })
		}
	}
	pub fn size(&self) -> Size {
		self.size
	}
	pub fn rows(&self) -> impl Iterator<Item = &[T]> {
		self.data.chunks_exact(self.size.width)
	}

	const fn index(&self, position: Position) -> Option<usize> {
		if self.size.contains(position) {
			Some((position.y() * self.size.width()) + position.x())
		} else {
			None
		}
	}
	pub fn swap(&mut self, a: Position, b: Position) {
		match (self.index(a), self.index(b)) {
			(Some(a), Some(b)) => self.data.swap(a, b),
			_ => (),
		}
	}
	pub fn get(&self, position: Position) -> Option<&T> {
		self.data.get(self.index(position)?)
	}
	pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
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
			data.extend(repeat(element).take(self.size.width + 2));
			for row in self.rows() {
				data.push(element.clone());
				data.extend(row.iter().copied());
				data.push(element.clone());
			}
			data.extend(repeat(element).take(self.size.width + 2));
			data
		};
		self.size.height += 2;
		self.size.width += 2;
	}
}
