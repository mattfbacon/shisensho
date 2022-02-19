use encde::Decode;

#[derive(Decode)]
#[repr(u8)]
pub enum Packet {
	#[encde(wire_tag = 0)]
	RequestStatus,
	#[encde(wire_tag = 1)]
	Ping(i64),
}
