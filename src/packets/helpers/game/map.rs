use super::super::wrappers::std::{PrefixedOption, PrefixedVec};
use super::chat::Chat;
use super::chunk::Position as ChunkPosition;
use encde::{Decode, Encode, Result as EResult};
use std::io::Write;

#[derive(Encode)]
pub struct MapIcon {
	icon_type: MapIconType,
	position: ChunkPosition<i8>,
	direction: i8,
	display_name: PrefixedOption<Chat>,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum MapIconType {
	WhiteArrow = 0,
	GreenArrow = 1,
	RedArrow = 2,
	BlueArrow = 3,
	WhiteCross = 4,
	RedPointer = 5,
	WhiteCircle = 6,
	SmallWhiteCircle = 7,
	Mansion = 8,
	Temple = 9,
	WhiteBanner = 10,
	OrangeBanner = 11,
	MagentaBanner = 12,
	LightBlueBanner = 13,
	YellowBanner = 14,
	LimeBanner = 15,
	PinkBanner = 16,
	GrayBanner = 17,
	LightGrayBanner = 18,
	CyanBanner = 19,
	PurpleBanner = 20,
	BlueBanner = 21,
	BrownBanner = 22,
	GreenBanner = 23,
	RedBanner = 24,
	BlackBanner = 25,
	TreasureMarker = 26,
}

pub struct MapUpdate {
	columns: u8,
	rows: u8,
	top_left: ChunkPosition<i8>,
	data: PrefixedVec<u8>,
}

impl Encode for MapUpdate {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		self.columns.encode(writer)?;
		if self.columns > 0 {
			self.rows.encode(writer)?;
			self.top_left.encode(writer)?;
			self.data.encode(writer)?;
		}
		Ok(())
	}
}
