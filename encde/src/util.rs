//! Utility functions related to encoding and decoding

use crate::{DecodeSized, Encode, Error};
use std::{cmp::min, io};

const EMPTY_SLICE: [u8; 1024] = [0u8; 1024];
/// Write `amount` zero bytes to `writer`
pub fn write_padding(writer: &mut dyn io::Write, mut amount: usize) -> io::Result<()> {
	while amount > EMPTY_SLICE.len() {
		writer.write_all(&EMPTY_SLICE)?;
		amount -= EMPTY_SLICE.len();
	}
	writer.write_all(&EMPTY_SLICE[0..amount])?;
	Ok(())
}

static mut EMPTY_MUT_SLICE: [u8; 1024] = [0u8; 1024];
/// Read and discard `amount` bytes from `reader`
pub fn read_padding(reader: &mut dyn io::Read, mut amount: usize) -> io::Result<()> {
	while amount > 0 {
		// Mutable statics are unsafe due to race conditions between threads, but we're using it as a bitbucket, so it doesn't matter
		amount -= unsafe { reader.read(&mut EMPTY_MUT_SLICE[0..min(amount, EMPTY_MUT_SLICE.len())])? }
	}
	Ok(())
}

/// Encode data into a Vec<u8> and return it
pub fn encode_to_vec(item: &dyn Encode) -> crate::Result<Vec<u8>> {
	let mut ret = Vec::new();
	item.encode(&mut ret)?;
	Ok(ret)
}

/// Decode data from a &[u8] and return the data, along with the number of bytes remaining
/// If the slice is too short, an Err variant will be returned
pub fn decode_from_slice<T: DecodeSized>(mut data: &[u8]) -> crate::Result<(T, usize)> {
	let len = data.len();
	let ret = T::decode_sized(&mut data, len)?;
	Ok((ret, data.len()))
}

/// Equivalent to `decode_from_slice`, except that an Err variant is returned if the entire slice was not used by `T`'s `decode` implementation.
pub fn decode_from_entire_slice<T: DecodeSized>(data: &[u8]) -> crate::Result<T> {
	let (decoded, amount_left): (T, usize) = decode_from_slice(data)?;
	if amount_left != 0 {
		Err(Error::UnexpectedLength {
			expected: data.len() - amount_left,
			actual: data.len(),
		})
	} else {
		Ok(decoded)
	}
}
