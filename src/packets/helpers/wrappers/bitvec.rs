use super::super::varint::VarInt;
use super::super::wrappers::std::PrefixedVec;
use super::super::wrappers::util::encode_encode_slice;
use bitvec::vec::BitVec;
use encde::{Decode, Encode, Result as EResult};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct PrefixedBitVec<T: bitvec::store::BitStore = u64>(pub BitVec<T>);

impl<T: Encode + bitvec::store::BitStore> Encode for PrefixedBitVec<T> {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		encode_encode_slice(writer, self.0.as_raw_slice())
	}
}

impl<T: Decode + bitvec::store::BitStore> Decode for PrefixedBitVec<T> {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let raw = PrefixedVec::<_, VarInt>::decode(reader)?.0;
		Ok(Self(BitVec::from_vec(raw)))
	}
}
