use crate::packets::helpers::varint::VarInt;
use crate::packets::helpers::wrappers::std::PrefixedString;
use encde::Decode;

#[derive(Decode, Debug)]
pub struct Handshake {
	pub protocol_version: VarInt,
	pub server_addr: PrefixedString,
	pub server_port: u16,
	/// status or login
	pub next_state: super::super::ProtocolState,
}

#[derive(Decode, Debug)]
#[repr(u8)]
pub enum Packet {
	#[encde(wire_tag = 0)]
	Handshake(Handshake),
}
