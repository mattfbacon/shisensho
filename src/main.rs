use anyhow::Context;

mod app;
mod logic;
mod ui;

fn main() -> anyhow::Result<()> {
	let signal_guard = graceful::SignalGuard::new();

	{
		let old_hook = std::panic::take_hook();
		std::panic::set_hook(Box::new(move |info: &std::panic::PanicInfo<'_>| {
			app::App::cleanup().unwrap();
			old_hook(info)
		}));
	}
	std::thread::spawn(|| {
		let mut app = app::App::new().context("Could not create app").unwrap();
		let ret = app.run();
		std::mem::drop(app);
		std::process::exit(match ret {
			Ok(()) => 0,
			Err(err) => {
				eprintln!("Error: {:?}", err);
				1
			}
		});
	});

	signal_guard.at_exit(move |_sig| app::App::cleanup().unwrap());
	Ok(())
}
