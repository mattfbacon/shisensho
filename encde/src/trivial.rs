//! # Trivial implementations of Encode and Decode
//!
//! This includes all integral types, and slices

use crate::{Decode, Encode, Error, Result};
use paste::paste;
use std::io::{Read, Write};

macro_rules! to_target_bytes {
	($value:expr) => {
		if cfg!(feature = "big_endian") {
			$value.to_be_bytes()
		} else {
			$value.to_le_bytes()
		}
	};
}

macro_rules! from_target_bytes {
	($type:ty, $buf:ident) => {
		if cfg!(feature = "big_endian") {
			<$type>::from_be_bytes($buf)
		} else {
			<$type>::from_le_bytes($buf)
		}
	};
}

macro_rules! integral_encde_impl {
	($type:ty, $test_val:expr) => {
		impl Encode for $type {
			fn encode(&self, writer: &mut dyn Write) -> Result<()> {
				writer.write_all(&to_target_bytes!(*self as $type)).map_err(Error::from)
			}
		}
		impl Decode for $type {
			fn decode(reader: &mut dyn Read) -> Result<Self> {
				let mut buf = [0u8; std::mem::size_of::<$type>()];
				reader.read_exact(&mut buf)?;
				Ok(from_target_bytes!($type, buf))
			}
		}
		paste! {
			#[cfg(test)]
			#[test]
			fn [< encode_ $type >]() {
				#![allow(clippy::unnecessary_cast)]
				let decoded = $test_val as $type;
				let encoded = $crate::util::encode_to_vec(&decoded).unwrap();
				assert_eq!(&encoded, &to_target_bytes!(decoded));
			}
			#[cfg(test)]
			#[test]
			fn [< decode_ $type >]() {
				#![allow(clippy::unnecessary_cast)]
				let value = $test_val as $type;
				let (decoded, amount_left): ($type, usize) = $crate::util::decode_from_slice(&to_target_bytes!(value)).unwrap();
				assert_eq!(amount_left, 0);
				assert_eq!(decoded, value);
			}
			#[cfg(test)]
			#[test]
			fn [< roundtrip_ $type >]() {
				#![allow(clippy::unnecessary_cast)]
				let value = $test_val as $type;
				let encoded = $crate::util::encode_to_vec(&value).unwrap();
				let (decoded, amount_left): ($type, usize) = $crate::util::decode_from_slice(&encoded).unwrap();
				assert_eq!(amount_left, 0);
				assert_eq!(value, decoded);
			}
		}
	};
}

integral_encde_impl!(u8, 4u8);
integral_encde_impl!(u16, 0x5843u16);
integral_encde_impl!(u32, 0x6fadcc3u32);
integral_encde_impl!(u64, 0x87fa641a4f06dfc9u64);

integral_encde_impl!(i8, -7i8);
integral_encde_impl!(i16, -30764i16);
integral_encde_impl!(i32, -47839028i32);
integral_encde_impl!(i64, -4784687647000839028i64);

integral_encde_impl!(f32, 7648.376f32);
integral_encde_impl!(f64, -167.6831f64);

impl Encode for bool {
	fn encode(&self, writer: &mut dyn Write) -> Result<()> {
		(*self as u8).encode(writer)
	}
}
impl Decode for bool {
	fn decode(reader: &mut dyn Read) -> Result<Self> {
		u8::decode(reader).map(|x| x > 0)
	}
}
#[cfg(test)]
#[test]
fn encode_bool() {
	let val = crate::util::encode_to_vec(&true).unwrap();
	assert_eq!(val.len(), 1);
	// does not need to be 1, just anything greater than 0
	assert!(val[0] > 0);
}
#[cfg(test)]
#[test]
fn decode_bool() {
	#![allow(clippy::bool_assert_comparison)]
	let data: [u8; 1] = [123];
	let (decoded, amount_left): (bool, usize) = crate::util::decode_from_slice(&data).unwrap();
	assert_eq!(amount_left, 0);
	assert_eq!(decoded, true);
}
#[cfg(test)]
#[test]
fn roundtrip_bool() {
	let val = true;
	let encoded = crate::util::encode_to_vec(&val).unwrap();
	let (decoded, amount_left): (bool, usize) = crate::util::decode_from_slice(&encoded).unwrap();
	assert_eq!(amount_left, 0);
	assert_eq!(decoded, val);
}

impl<T: Encode, const N: usize> Encode for [T; N] {
	fn encode(&self, writer: &mut dyn Write) -> Result<()> {
		for item in self {
			item.encode(writer)?;
		}
		Ok(())
	}
}
impl<T: Decode, const N: usize> Decode for [T; N] {
	fn decode(reader: &mut dyn Read) -> Result<Self> {
		unsafe {
			#![allow(clippy::uninit_assumed_init)]
			let mut ret: [T; N] = std::mem::MaybeUninit::uninit().assume_init();
			for item in ret.iter_mut() {
				std::ptr::write(item, T::decode(reader)?);
			}
			Ok(ret)
		}
	}
}

impl<T: Encode> Encode for &T {
	fn encode(&self, writer: &mut dyn Write) -> Result<()> {
		(*self).encode(writer)
	}
}

impl Encode for () {
	fn encode(&self, _writer: &mut dyn Write) -> Result<()> {
		Ok(())
	}
}
impl Decode for () {
	fn decode(_reader: &mut dyn Read) -> Result<Self> {
		Ok(())
	}
}
