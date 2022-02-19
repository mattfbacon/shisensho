use encde::{Decode, Encode, Result as EResult};
use serde::Serialize;
use std::io::{Read, Write};

pub struct Uuid(pub uuid::Uuid);

impl Encode for Uuid {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		writer.write_all(&self.0.as_u128().to_be_bytes())?;
		Ok(())
	}
}

impl Decode for Uuid {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let mut buf: uuid::Bytes = [0u8; 16];
		reader.read_exact(&mut buf)?;
		// PANICS: from_slice only panics if the buffer is the wrong length and we used the type from the `uuid` crate directly to ensure the correct size.
		Ok(Self(uuid::Uuid::from_slice(&buf).unwrap()))
	}
}

impl std::fmt::Display for Uuid {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.to_hyphenated().fmt(formatter)
	}
}

impl Serialize for Uuid {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&self.to_string())
	}
}
