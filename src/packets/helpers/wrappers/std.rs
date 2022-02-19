use super::super::varint::VarInt;
use super::util::{encode_encode_slice, encode_u8_slice, encode_usize_as_varint};
use encde::{Decode, Encode, Result as EResult};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct PrefixedArray<T, const N: usize>(pub [T; N]);
#[derive(Debug)]
pub struct PrefixedBytes(pub Vec<u8>);
#[derive(Debug)]
pub struct PrefixedBorrowedBytes<'a>(pub &'a [u8]);
#[derive(Debug)]
pub struct PrefixedString(pub String);
#[derive(Debug)]
pub struct UnprefixedBytes(pub Vec<u8>);
#[derive(Debug)]
pub struct PrefixedVec<T, SizeType = VarInt>(pub Vec<T>, std::marker::PhantomData<SizeType>)
where
	SizeType: Encode + Decode + TryInto<usize> + 'static,
	<SizeType as TryInto<usize>>::Error: std::error::Error;
#[derive(Debug)]
pub struct PrefixedOption<T>(pub Option<T>);
impl<T, SizeType> PrefixedVec<T, SizeType>
where
	SizeType: Encode + Decode + TryInto<usize> + 'static,
	<SizeType as TryInto<usize>>::Error: std::error::Error,
{
	pub fn new(underlying: Vec<T>) -> Self {
		Self(underlying, std::marker::PhantomData)
	}
}

impl<T: Encode, const N: usize> Encode for PrefixedArray<T, N> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_usize_as_varint(writer, N)?;
		self.0.encode(writer)?;
		Ok(())
	}
}

impl<T: Decode, const N: usize> Decode for PrefixedArray<T, N> {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let len = VarInt::decode(reader)?.0.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?;
		if len != N {
			return Err(encde::Error::UnexpectedLength { expected: N, actual: len });
		}
		Ok(Self(<[T; N]>::decode(reader)?))
	}
}

impl Encode for PrefixedString {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_u8_slice(writer, self.0.as_bytes())
	}
}

impl Decode for PrefixedString {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let len = VarInt::decode(reader)?;
		let len: usize = len.0.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?;
		let mut buffer = vec![0u8; len];
		reader.read_exact(&mut buffer)?;
		Ok(Self(String::from_utf8(buffer).map_err(|err| encde::Error::Custom(Box::new(err)))?))
	}
}

impl Encode for PrefixedBorrowedBytes<'_> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_u8_slice(writer, self.0)
	}
}

impl Encode for PrefixedBytes {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_u8_slice(writer, self.0.as_slice())
	}
}

impl Decode for PrefixedBytes {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let len = VarInt::decode(reader)?;
		let len: usize = len.0.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?;
		let mut buffer = vec![0u8; len];
		reader.read_exact(&mut buffer)?;
		Ok(Self(buffer))
	}
}

impl Encode for UnprefixedBytes {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		writer.write_all(self.0.as_slice())?;
		Ok(())
	}
}
impl encde::DecodeSized for UnprefixedBytes {
	fn decode_sized(reader: &mut dyn Read, size: usize) -> EResult<Self> {
		let mut ret = vec![0u8; size];
		reader.read_exact(ret.as_mut_slice())?;
		Ok(Self(ret))
	}
}

impl<T: Encode, SizeType> Encode for PrefixedVec<T, SizeType>
where
	SizeType: Encode + Decode + TryInto<usize> + 'static,
	<SizeType as TryInto<usize>>::Error: std::error::Error,
{
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_encode_slice(writer, self.0.as_slice())
	}
}

impl<T: Decode, SizeType> Decode for PrefixedVec<T, SizeType>
where
	SizeType: Encode + Decode + TryInto<usize> + 'static,
	<SizeType as TryInto<usize>>::Error: std::error::Error + Send + Sync,
{
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let len = SizeType::decode(reader)?;
		let len: usize = len.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?;
		let mut ret = Vec::with_capacity(len);
		for _ in 0..len {
			ret.push(T::decode(reader)?);
		}
		Ok(Self::new(ret))
	}
}

impl<T: Encode> Encode for PrefixedOption<T> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		self.0.is_some().encode(writer)?;
		match &self.0 {
			Some(inner) => inner.encode(writer)?,
			None => (),
		}
		Ok(())
	}
}

impl<T: Decode> Decode for PrefixedOption<T> {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let is_some = bool::decode(reader)?;
		Ok(Self(if is_some { Some(T::decode(reader)?) } else { None }))
	}
}
