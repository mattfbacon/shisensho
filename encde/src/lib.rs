#![warn(missing_docs)]

//! # Encde
//! Simple, **predictable** data encoding and decoding, including derive macros.
//!
//! A few things to note:
//!
//! This crate is little-endian unless the "big_endian" feature is enabled.
//! However, endianness can also be specified on the aggregate (enum/struct) or item (field/variant) level when using derive macros.

use std::{io, result};

#[cfg(feature = "derive")]
pub use encde_derive::{Decode, Encode};

pub mod trivial;
pub mod util;

/// An integer that can be `Signed` or `Unsigned`, able to contain the full range of either a `u64` or an `i64`
#[derive(Debug)]
pub enum UnknownSignInt {
	/// The signed variant, using an `i64`
	Signed(i64),
	/// The unsigned variant, using a `u64`
	Unsigned(u64),
}
impl std::fmt::Display for UnknownSignInt {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			UnknownSignInt::Signed(v) => v.fmt(formatter),
			UnknownSignInt::Unsigned(v) => v.fmt(formatter),
		}
	}
}
macro_rules! into_unknown_sign_int_impl {
	($variant:ident using $actual:ty, $ty:ty) => {
		impl From<$ty> for UnknownSignInt {
			fn from(x: $ty) -> Self {
				Self::$variant(x as $actual)
			}
		}
	};
	($variant:ident using $actual:ty, $($ty:ty),+) => {
		$(into_unknown_sign_int_impl!($variant using $actual, $ty);)*
	};
}

into_unknown_sign_int_impl!(Signed using i64, i8, i16, i32, i64, isize);
into_unknown_sign_int_impl!(Unsigned using u64, u8, u16, u32, u64, usize);

/// The possible errors when encoding or decoding
#[derive(Debug)]
pub enum Error {
	/// An underlying error occurred when writing to a `std::io::Write` or reading to a `std::io::Read`
	Io(io::Error),
	/// The length of the underlying data did not match the expected length
	UnexpectedLength {
		/// The expected length of the data
		expected: usize,
		/// The actual length of the data. If `actual < expected`, not enough data was received. If `actual > expected`, too much data was received.
		actual: usize,
	},
	/// When decoding an enum, the decoded discriminant did not match the discriminant of any of the enum's variants
	UnrecognizedEnumDiscriminant {
		/// The name of the enum that was being decoded
		enum_name: &'static str,
		/// All the possible discriminants
		expected: &'static [UnknownSignInt],
		/// The discriminant that was received
		actual: UnknownSignInt,
	},
	/// Data was successfully received but was not valid for the decoded format
	/// Similar to HTTP response code 415 or 422
	InvalidFormat {
		/// The name of the expected format
		format_name: &'static str,
	},
	/// A static custom error
	CustomStr(&'static str),
	/// A dynamic custom error
	Custom(Box<dyn std::error::Error + Sync + Send + 'static>),
}

impl std::fmt::Display for Error {
	fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Io(e) => write!(formatter, "IO error: {}", e),
			Self::UnexpectedLength { expected, actual } => write!(formatter, "Length ({}) did not match expected ({})", actual, expected),
			Self::UnrecognizedEnumDiscriminant { enum_name, expected, actual } => write!(formatter, "Unrecognized enum discriminant {} for enum {}; expected one of {:?}", actual, enum_name, expected),
			Self::InvalidFormat { format_name } => write!(formatter, "Invalid format for {}", format_name),
			Self::CustomStr(message) => formatter.write_str(message),
			Self::Custom(message) => write!(formatter, "{}", message),
		}
	}
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::Io(underlying) => Some(underlying),
			Self::Custom(underlying) => Some(&**underlying),
			_ => None,
		}
	}
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Self::Io(e)
	}
}
impl From<std::str::Utf8Error> for Error {
	fn from(_: std::str::Utf8Error) -> Self {
		Self::InvalidFormat { format_name: "UTF-8" }
	}
}

/// A `Result` where the error type is `Error`
pub type Result<T> = result::Result<T, Error>;

/// Allows a type to be encoded into a `std::io::Write`
pub trait Encode {
	/// Encode the type into a `std::io::Write`
	fn encode(&self, writer: &mut dyn io::Write) -> Result<()>;
}

/// Allows a type to be decoded out of a `std::io::Read`
pub trait Decode: Sized {
	/// Decode the type from a `std::io::Read`
	fn decode(reader: &mut dyn io::Read) -> Result<Self>;
}

/// Allows a type to be decoded out of a `std::io::Read` when the size is known beforehand
pub trait DecodeSized: Sized {
	/// Decode the type from a `std::io::Read` with the size known beforehand
	fn decode_sized(reader: &mut dyn io::Read, size: usize) -> Result<Self>;
}

impl<T: Decode> DecodeSized for T {
	fn decode_sized(reader: &mut dyn io::Read, _size: usize) -> Result<Self> {
		T::decode(reader)
	}
}
