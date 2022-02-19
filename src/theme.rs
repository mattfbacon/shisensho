use cursive::theme::{BorderStyle, Color, Palette, PaletteColor, Theme};

pub fn theme() -> Theme {
	let mut palette = Palette::default();
	palette[PaletteColor::Background] = Color::TerminalDefault;
	palette[PaletteColor::Shadow] = Color::TerminalDefault;
	palette[PaletteColor::View] = Color::TerminalDefault;
	palette[PaletteColor::Primary] = Color::TerminalDefault;
	palette[PaletteColor::Secondary] = Color::TerminalDefault;
	palette[PaletteColor::Tertiary] = Color::TerminalDefault;
	palette[PaletteColor::TitlePrimary] = Color::TerminalDefault;
	palette[PaletteColor::TitleSecondary] = Color::TerminalDefault;
	palette[PaletteColor::Highlight] = Color::TerminalDefault;
	palette[PaletteColor::HighlightInactive] = Color::TerminalDefault;
	palette[PaletteColor::HighlightText] = Color::TerminalDefault;
	Theme {
		shadow: false,
		borders: BorderStyle::Simple,
		palette,
	}
}
