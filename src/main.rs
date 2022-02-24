use cursive::traits::Nameable;
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
		let board = board::Board::default().with_name("board");
		let board = board::CenterView::new(board);
		let elapsed = elapsed::Elapsed::new().with_name("elapsed");
		let mut ret = LinearLayout::vertical();
		ret.add_child(board);
		ret.add_child(elapsed);
		ret
	});
	siv.add_global_callback('q', |siv| {
		siv.quit();
	});
	siv.add_global_callback('r', |siv| {
		siv.call_on_name("board", |current_board| *current_board = board::Board::default());
		siv.call_on_name("elapsed", |current_elapsed| *current_elapsed = elapsed::Elapsed::new());
	});
	siv.run();

	Ok(())
}
