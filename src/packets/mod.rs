//! Packets are divided between the states in which they will be sent/received
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

pub mod cipher;
pub mod helpers;

#[derive(encde::Encode, encde::Decode, Debug)]
#[repr(u8)]
pub enum ProtocolState {
	Handshake = 0,
	Status = 1,
	Login = 2,
	Play = 3,
}
