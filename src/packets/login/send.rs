use crate::packets::helpers::game::chat::Chat;
use crate::packets::helpers::varint::VarInt;
use crate::packets::helpers::wrappers::std::{PrefixedArray, PrefixedBorrowedBytes, PrefixedString};
use crate::packets::helpers::wrappers::uuid::Uuid;
use encde::Encode;

/// This packet is optional and not sending it means to not compress.
/// Thus, we don't need to send it until we've implemented compression.
#[derive(Encode)]
pub struct SetCompression {
	/// Maximum packet size before it's compressed. If <= 0, disable compression.
	threshold: VarInt,
}

// No LoginPluginRequest for now

#[derive(Encode)]
#[repr(u8)]
pub enum Packet<'a> {
	#[encde(wire_tag = 0)]
	Disconnect { reason: Chat },
	#[encde(wire_tag = 1)]
	Encryption {
		server_id: PrefixedString,
		public_key: PrefixedBorrowedBytes<'a>,
		verify_token: PrefixedArray<u8, 4>,
	},
	#[encde(wire_tag = 2)]
	LoginSuccess { uuid: Uuid, username: PrefixedString },
	#[encde(wire_tag = 3)]
	SetCompression(SetCompression),
	// #[encde(wire_tag = 4)]
	// LoginPluginRequest(LoginPluginRequest),
}
