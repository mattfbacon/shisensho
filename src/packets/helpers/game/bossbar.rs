use super::chat::Chat;
use encde::{Decode, Encode};

#[derive(Encode)]
#[repr(u8)]
pub enum UpdateType {
	#[encde(wire_tag = 0)]
	Add {
		title: Chat,
		/// from 0 to 1
		health: f32,
		color: Color,
		notches: Notches,
		/// bit mask (TODO custom type)
		/// 1 = darken sky
		/// 2 = dragon bar
		/// 4 = create fog
		flags: u8,
	},
	#[encde(wire_tag = 1)]
	Remove,
	#[encde(wire_tag = 2)]
	UpdateHealth(f32),
	#[encde(wire_tag = 3)]
	UpdateTitle(Chat),
	#[encde(wire_tag = 4)]
	UpdateStyle { color: Color, notches: Notches },
	/// TODO custom type
	#[encde(wire_tag = 5)]
	UpdateFlags(u8),
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum Color {
	Pink = 0,
	Blue = 1,
	Red = 2,
	Green = 3,
	Yellow = 4,
	Purple = 5,
	White = 6,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum Notches {
	None = 0,
	Six = 1,
	Ten = 2,
	Twelve = 3,
	Twenty = 4,
}
