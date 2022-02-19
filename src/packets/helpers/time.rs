use super::varint::VarInt;
use encde::{Decode, Encode, Result as EResult};
use std::io::{Read, Write};

pub struct Milliseconds(std::time::Duration);

impl Encode for Milliseconds {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		super::varint::VarLong(self.0.as_millis().try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?).encode(writer)
	}
}

impl Decode for Milliseconds {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let value = super::varint::VarLong::decode(reader)?.0;
		Ok(Self(std::time::Duration::from_millis(value.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?)))
	}
}

pub struct Seconds(std::time::Duration);

impl Encode for Seconds {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		super::varint::VarLong(self.0.as_secs().try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?).encode(writer)
	}
}

impl Decode for Seconds {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let value = super::varint::VarLong::decode(reader)?.0;
		Ok(Self(std::time::Duration::from_secs(value.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?)))
	}
}

// TODO custom types with Duration-based constructors
pub type Ticks64 = u64;
pub type Ticks32 = u32;
pub type TicksVarInt = VarInt;
