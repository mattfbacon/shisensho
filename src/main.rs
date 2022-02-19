use cursive::views::LinearLayout;
use cursive::{Cursive, CursiveExt};

mod board;
mod elapsed;
mod ext;
mod matrix;
mod theme;
mod tile;

fn main() -> anyhow::Result<()> {
	let mut siv = Cursive::new();

	siv.set_theme(theme::theme());
	siv.set_autorefresh(true);
	siv.add_fullscreen_layer({
		let board = board::Board::default();
		let board = board::CenterView::new(board);
		let elapsed = elapsed::Elapsed::new();
		let mut ret = LinearLayout::vertical();
		ret.add_child(board);
		ret.add_child(elapsed);
		ret
	});
	siv.run();

	Ok(())
}
