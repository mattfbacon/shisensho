use crate::ext::vec2::*;
use cursive::{Printer, View};
use std::time::Instant;

pub struct Elapsed(Instant);

impl Elapsed {
	pub fn new() -> Self {
		Self(Instant::now())
	}
}

impl View for Elapsed {
	fn draw(&self, printer: &Printer<'_, '_>) {
		let elapsed = self.0.elapsed().as_secs();
		let (minutes, seconds) = (elapsed / 60, elapsed % 60);
		let text = format!("{}:{:02} elapsed", minutes, seconds);
		printer.print(cursive::Vec2::new(0, 0), &text)
	}
	fn required_size(&mut self, constraint: Vec2) -> Vec2 {
		constraint.with_y(1)
	}
}
