use crate::packets::helpers::wrappers::std::{PrefixedBytes, PrefixedString};
use encde::Decode;

#[derive(Decode, Debug)]
pub struct Encryption {
	pub shared_secret: PrefixedBytes,
	pub verify_token: PrefixedBytes,
}

// TODO LoginPluginResponse

#[derive(Decode, Debug)]
#[repr(u8)]
pub enum Packet {
	#[encde(wire_tag = 0)]
	LoginStart { username: PrefixedString },
	#[encde(wire_tag = 1)]
	Encryption(Encryption),
	// #[encde(wire_tag = 2)]
	// LoginPluginResponse(LoginPluginResponse),
}
