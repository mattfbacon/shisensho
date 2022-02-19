use encde::{Decode, Encode, Result as EResult};
use std::io::{Read, Write};

#[derive(Encode, Decode)]
pub struct UnpackedPosition<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}
pub type F32Position = UnpackedPosition<f32>;
pub type F64Position = UnpackedPosition<f64>;

// XXX is it better to store the position as packed or unpacked?
pub struct PackedPosition {
	x: i32,
	y: i16,
	z: i32,
}
impl PackedPosition {
	pub fn new(x: i32, y: i16, z: i32) -> Self {
		Self { x, y, z }
	}
}
impl Encode for PackedPosition {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		// 26 MSBs = x
		// 26 middle bits = z
		// 12 LSBs = y
		let out = (((self.x as u32 & 0x3ffffff) as u64) << 38) | (((self.z as u32 & 0x3fffff) as u64) << 12) | ((self.y as u16 & 0xfff) as u64);
		out.encode(writer)
	}
}
impl Decode for PackedPosition {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let raw = u64::decode(reader)?;
		Ok(Self {
			x: (raw >> 38) as i32,
			y: (raw & 0xfff) as i16,
			z: ((raw >> 12) & 0x3ffffff) as i32,
		})
	}
}

/// coordinates are in eighths
/// TODO write constructor once use case is known
#[derive(Encode, Decode)]
pub struct EffectPosition(UnpackedPosition<i32>);
