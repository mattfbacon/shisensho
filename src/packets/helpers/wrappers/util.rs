use super::super::varint::VarInt;
use encde::{Encode, Result as EResult};
use std::io::Write;

pub fn encode_usize_as_varint(writer: &mut dyn Write, val: usize) -> EResult<()> {
	VarInt(val.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?).encode(writer)
}
pub fn encode_encode_slice<T: Encode>(writer: &mut dyn Write, slice: &[T]) -> EResult<()> {
	encode_usize_as_varint(writer, slice.len())?;
	for item in slice.iter() {
		item.encode(writer)?;
	}
	Ok(())
}
pub fn encode_u8_slice(writer: &mut dyn Write, slice: &[u8]) -> EResult<()> {
	encode_usize_as_varint(writer, slice.len())?;
	writer.write_all(slice)?;
	Ok(())
}
