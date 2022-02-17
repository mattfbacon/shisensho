use tui::backend::CrosstermBackend;

pub type Terminal = tui::Terminal<CrosstermBackend<std::io::Stdout>>;
pub type Frame<'a> = tui::Frame<'a, CrosstermBackend<std::io::Stdout>>;

pub struct App {
	terminal: Terminal,
	ui: crate::ui::Ui,
}

impl App {
	fn backend() -> anyhow::Result<Terminal> {
		let stdout = std::io::stdout();
		let backend = CrosstermBackend::new(stdout);
		Ok(tui::Terminal::new(backend)?)
	}

	pub fn new() -> anyhow::Result<Self> {
		use crossterm::{
			event::EnableMouseCapture,
			execute,
			terminal::{enable_raw_mode, EnterAlternateScreen},
		};

		enable_raw_mode()?;
		let mut backend = Self::backend()?;
		execute!(backend.backend_mut(), EnterAlternateScreen, EnableMouseCapture)?;

		Ok(App { terminal: backend, ui: Default::default() })
	}

	pub fn cleanup() -> anyhow::Result<()> {
		use crossterm::{
			event::DisableMouseCapture,
			execute,
			terminal::{disable_raw_mode, LeaveAlternateScreen},
		};

		disable_raw_mode()?;
		let mut backend = Self::backend()?;
		execute!(backend.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
		backend.show_cursor()?;
		Ok(())
	}

	pub fn run(&mut self) -> anyhow::Result<()> {
		use crate::ui::ControlFlow;
		use crossterm::event;
		use std::time::{Duration, Instant};

		let tick_rate = Duration::from_millis(10);
		let mut last_tick = Instant::now();
		loop {
			// `Duration::from_secs` is a `const fn` so the closure overhead of `unwrap_or_else` is pointless
			let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or(Duration::from_secs(0));

			let mut should_view = false;
			if event::poll(timeout)? {
				match self.ui.update(event::read()?) {
					ControlFlow::Break(why) => break why,
					ControlFlow::Continue => (),
				};
				should_view = true;
			}
			if last_tick.elapsed() >= tick_rate {
				match self.ui.tick() {
					ControlFlow::Break(why) => break why,
					ControlFlow::Continue => (),
				};
				last_tick = Instant::now();
				should_view = true;
			}
			if should_view {
				self.terminal.draw(|frame| self.ui.view(frame))?;
			}
		}
	}
}

impl Drop for App {
	fn drop(&mut self) {
		Self::cleanup().unwrap()
	}
}
