use encde::{Decode, Encode, Result as EResult};
use serde::Serialize;
use std::io::{Read, Write};

pub struct NbtData<T>(pub T);
pub struct NbtBlob(nbt::Blob);

impl Encode for NbtBlob {
	fn encode(&self, mut writer: &mut dyn Write) -> EResult<()> {
		self.0.to_writer(&mut writer).map_err(|err| encde::Error::Custom(Box::new(err)))
	}
}

impl Decode for NbtBlob {
	fn decode(mut reader: &mut dyn Read) -> EResult<Self> {
		Ok(Self(nbt::Blob::from_reader(&mut reader).map_err(|err| encde::Error::Custom(Box::new(err)))?))
	}
}

impl<T: Serialize> Encode for NbtData<T> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		nbt::to_writer(writer, &self.0, None).map_err(|err| encde::Error::Custom(Box::new(err)))
	}
}

impl<T: serde::de::DeserializeOwned> Decode for NbtData<T> {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		Ok(Self(nbt::from_reader(reader).map_err(|err| encde::Error::Custom(Box::new(err)))?))
	}
}
