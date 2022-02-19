use crate::ext::vec2::*;
use cursive::direction::Direction;
use cursive::event::{AnyCb, Event, EventResult};
use cursive::view::{CannotFocus, Selector, View, ViewNotFound, ViewWrapper};
use cursive::{Printer, Rect};

pub struct CenterView<V: View> {
	inner: V,
	inner_position: Rect,
}

impl<V: View> CenterView<V> {
	pub fn new(inner: V) -> Self {
		CenterView {
			inner,
			inner_position: Rect::from_point(Vec2::new(0, 0)),
		}
	}
}

impl<V: View> ViewWrapper for CenterView<V> {
	type V = V;

	fn with_view<F: FnOnce(&Self::V) -> R, R>(&self, f: F) -> Option<R> {
		Some(f(&self.inner))
	}
	fn with_view_mut<F: FnOnce(&mut Self::V) -> R, R>(&mut self, f: F) -> Option<R> {
		Some(f(&mut self.inner))
	}

	fn into_inner(self) -> Result<Self::V, Self>
	where
		Self::V: Sized,
	{
		Ok(self.inner)
	}

	fn wrap_draw(&self, printer: &Printer<'_, '_>) {
		let printer = printer.windowed(self.inner_position);
		self.inner.draw(&printer)
	}
	fn wrap_required_size(&mut self, request: Vec2) -> Vec2 {
		let inner_size = self.inner.required_size(request);
		self.inner_position = Rect::from_size(Vec2::new(0, 0), inner_size);
		Vec2::max(request, inner_size)
	}
	fn wrap_on_event(&mut self, mut event: Event) -> EventResult {
		event.relativize(self.inner_position.top_left());
		self.inner.on_event(event)
	}
	fn wrap_layout(&mut self, outer: Vec2) {
		self.inner.layout(self.inner_position.size());
		// center the inner view
		self.inner_position = self.inner_position + Vec2::new(outer.x.saturating_sub(self.inner_position.width()) / 2, outer.y.saturating_sub(self.inner_position.height()) / 2);
	}
	fn wrap_take_focus(&mut self, source: Direction) -> Result<EventResult, CannotFocus> {
		self.inner.take_focus(source)
	}
	fn wrap_call_on_any<'a>(&mut self, selector: &Selector<'_>, callback: AnyCb<'a>) {
		self.inner.call_on_any(selector, callback)
	}
	fn wrap_focus_view(&mut self, selector: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
		self.inner.focus_view(selector)
	}
	fn wrap_needs_relayout(&self) -> bool {
		self.inner.needs_relayout()
	}
	fn wrap_important_area(&self, size: Vec2) -> Rect {
		self.inner.important_area(size)
	}
}
