#![cfg(test)]

use encde::{Decode, Encode};

macro_rules! generate_test {
	($name:ident for $ty:ident with encoding $expected:expr) => {
		mod $name {
			use super::$ty;
			use encde::util::{decode_from_entire_slice, encode_to_vec};

			#[test]
			fn encode() {
				let value: $ty = $ty::default();
				let encoded = encode_to_vec(&value).unwrap();
				assert_eq!(&encoded, &$expected);
			}
			#[test]
			fn decode() {
				let encoded = $expected;
				let decoded: $ty = decode_from_entire_slice(&encoded).unwrap();
				assert_eq!(decoded, $ty::default());
			}
			#[test]
			fn roundtrip() {
				let value: $ty = $ty::default();
				let encoded = encode_to_vec(&value).unwrap();
				let decoded: $ty = decode_from_entire_slice(&encoded).unwrap();
				assert_eq!(value, decoded);
			}
		}
	};
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct BasicStruct {
	a: u8,
	b: u8,
	c: u8,
}
impl Default for BasicStruct {
	fn default() -> Self {
		Self { a: 1, b: 2, c: 3 }
	}
}
#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(i16)]
enum BasicEnum {
	AVar = -3,
	BVar = 0x7,
	CVar = 0b101,
}
impl Default for BasicEnum {
	fn default() -> Self {
		Self::AVar
	}
}

generate_test!(basic_struct for BasicStruct with encoding [0x1, 0x2, 0x3]);
generate_test!(basic_enum for BasicEnum with encoding [0xfd, 0xff]);
#[test]
fn invalid_enum_discriminant() {
	let invalid = [0x12, 0x34];
	assert!(::encde::util::decode_from_entire_slice::<BasicEnum>(&invalid).is_err());
}

#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(u32)]
enum ComplicatedDiscriminant {
	#[encde(wire_tag = 3)]
	AVar {
		x: u8,
		#[encde(pad_before = 1)]
		y: u16,
	},
	#[encde(wire_tag)]
	BVar(u8),
	#[encde(wire_tag = sync)]
	CVar,
}
impl Default for ComplicatedDiscriminant {
	fn default() -> Self {
		Self::AVar { x: 1, y: 2 }
	}
}
generate_test!(complicated_discriminant for ComplicatedDiscriminant with encoding [0x3, 0x0, 0x0, 0x0, 0x1, 0x0, 0x2, 0x0]);
