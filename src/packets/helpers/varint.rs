use encde::{Decode, Encode, Result as EResult};
use std::io::{Read, Write};

// because inherent associated types (like everything else useful) are unstable
pub type VarIntUnderlying = i32;
type VarIntUnderlyingUnsigned = u32;
pub type VarLongUnderlying = i64;
type VarLongUnderlyingUnsigned = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt(pub VarIntUnderlying);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarLong(pub VarLongUnderlying);

impl TryFrom<VarInt> for usize {
	type Error = std::num::TryFromIntError;
	fn try_from(val: VarInt) -> Result<Self, Self::Error> {
		val.0.try_into()
	}
}
impl TryFrom<VarLong> for usize {
	type Error = std::num::TryFromIntError;
	fn try_from(val: VarLong) -> Result<Self, Self::Error> {
		val.0.try_into()
	}
}

const DATA_BITS: usize = 7;
const EXTEND_BIT: u8 = 1 << DATA_BITS;
const DATA_MASK: u8 = !EXTEND_BIT;

impl VarInt {
	const MAX_BYTES: usize = 5;
}

impl VarLong {
	const MAX_BYTES: usize = 10;
}

macro_rules! impl_var {
	($ty:tt, $under:tt, $under_u:tt) => {
		impl Encode for $ty {
			fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
				let mut val = self.0 as $under_u; // so >> shifts the sign bit
				loop {
					if val < EXTEND_BIT.try_into().unwrap() {
						writer.write_all(&[val as u8])?;
						return Ok(());
					}
					writer.write_all(&[(val & <$under_u>::from(DATA_MASK)) as u8 | EXTEND_BIT])?;
					val = val.wrapping_shr(DATA_BITS.try_into().unwrap());
				}
			}
		}
		impl Decode for $ty {
			fn decode(writer: &mut dyn Read) -> EResult<Self> {
				let mut ret: $under_u = 0;
				let mut byte_index = 0usize;
				loop {
					let current_byte = u8::decode(writer)?;
					ret |= ((current_byte & DATA_MASK) as $under_u) << (byte_index * DATA_BITS);
					byte_index += 1;
					if byte_index > Self::MAX_BYTES {
						panic!(stringify!(Received $ty value is too large));
					}
					if (current_byte & EXTEND_BIT) != EXTEND_BIT {
						return Ok($ty(ret as $under));
					}
				}
			}
		}
	};
}

impl_var!(VarInt, VarIntUnderlying, VarIntUnderlyingUnsigned);
impl_var!(VarLong, VarLongUnderlying, VarLongUnderlyingUnsigned);

#[cfg(test)]
mod test {
	use super::{VarInt, VarLong};
	use encde::util::{decode_from_entire_slice, encode_to_vec};

	#[test]
	fn encode_varint() {
		assert_eq!(&encode_to_vec(&VarInt(0)).unwrap(), &[0]);
		assert_eq!(&encode_to_vec(&VarInt(0)).unwrap(), &[0]);
		assert_eq!(&encode_to_vec(&VarInt(1)).unwrap(), &[1]);
		assert_eq!(&encode_to_vec(&VarInt(2)).unwrap(), &[2]);
		assert_eq!(&encode_to_vec(&VarInt(127)).unwrap(), &[127]);
		assert_eq!(&encode_to_vec(&VarInt(128)).unwrap(), &[128, 1]);
		assert_eq!(&encode_to_vec(&VarInt(255)).unwrap(), &[255, 1]);
		assert_eq!(&encode_to_vec(&VarInt(25565)).unwrap(), &[221, 199, 1]);
		assert_eq!(&encode_to_vec(&VarInt(2097151)).unwrap(), &[255, 255, 127]);
		assert_eq!(&encode_to_vec(&VarInt(2147483647)).unwrap(), &[255, 255, 255, 255, 7]);
		assert_eq!(&encode_to_vec(&VarInt(-1)).unwrap(), &[255, 255, 255, 255, 15]);
		assert_eq!(&encode_to_vec(&VarInt(-2147483648)).unwrap(), &[128, 128, 128, 128, 8]);
	}

	#[test]
	fn encode_varlong() {
		assert_eq!(&encode_to_vec(&VarLong(0)).unwrap(), &[0]);
		assert_eq!(&encode_to_vec(&VarLong(1)).unwrap(), &[1]);
		assert_eq!(&encode_to_vec(&VarLong(2)).unwrap(), &[2]);
		assert_eq!(&encode_to_vec(&VarLong(127)).unwrap(), &[127]);
		assert_eq!(&encode_to_vec(&VarLong(128)).unwrap(), &[128, 1]);
		assert_eq!(&encode_to_vec(&VarLong(255)).unwrap(), &[255, 1]);
		assert_eq!(&encode_to_vec(&VarLong(2147483647)).unwrap(), &[255, 255, 255, 255, 7]);
		assert_eq!(&encode_to_vec(&VarLong(9223372036854775807)).unwrap(), &[255, 255, 255, 255, 255, 255, 255, 255, 127]);
		assert_eq!(&encode_to_vec(&VarLong(-1)).unwrap(), &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);
		assert_eq!(&encode_to_vec(&VarLong(-2147483648)).unwrap(), &[128, 128, 128, 128, 248, 255, 255, 255, 255, 1]);
		assert_eq!(&encode_to_vec(&VarLong(-9223372036854775808)).unwrap(), &[128, 128, 128, 128, 128, 128, 128, 128, 128, 1]);
	}

	#[test]
	fn decode_varint() {
		assert_eq!(decode_from_entire_slice::<VarInt>(&[0]).unwrap(), VarInt(0));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[0]).unwrap(), VarInt(0));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[1]).unwrap(), VarInt(1));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[2]).unwrap(), VarInt(2));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[127]).unwrap(), VarInt(127));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[128, 1]).unwrap(), VarInt(128));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[255, 1]).unwrap(), VarInt(255));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[221, 199, 1]).unwrap(), VarInt(25565));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[255, 255, 127]).unwrap(), VarInt(2097151));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[255, 255, 255, 255, 7]).unwrap(), VarInt(2147483647));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[255, 255, 255, 255, 15]).unwrap(), VarInt(-1));
		assert_eq!(decode_from_entire_slice::<VarInt>(&[128, 128, 128, 128, 8]).unwrap(), VarInt(-2147483648));
	}

	#[test]
	fn decode_varlong() {
		assert_eq!(decode_from_entire_slice::<VarLong>(&[0]).unwrap(), VarLong(0));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[1]).unwrap(), VarLong(1));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[2]).unwrap(), VarLong(2));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[127]).unwrap(), VarLong(127));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[128, 1]).unwrap(), VarLong(128));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[255, 1]).unwrap(), VarLong(255));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[255, 255, 255, 255, 7]).unwrap(), VarLong(2147483647));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[255, 255, 255, 255, 255, 255, 255, 255, 127]).unwrap(), VarLong(9223372036854775807));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 1]).unwrap(), VarLong(-1));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[128, 128, 128, 128, 248, 255, 255, 255, 255, 1]).unwrap(), VarLong(-2147483648));
		assert_eq!(decode_from_entire_slice::<VarLong>(&[128, 128, 128, 128, 128, 128, 128, 128, 128, 1]).unwrap(), VarLong(-9223372036854775808));
	}
}
