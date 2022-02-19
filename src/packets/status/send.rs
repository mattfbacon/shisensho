use crate::packets::helpers::wrappers::{json::Json, uuid::Uuid};
use encde::Encode;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusReply<'a> {
	pub version: StatusVersion,
	pub players: StatusPlayers,
	pub description: StatusDescription<'a>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub favicon: Option<&'a str>,
}

#[derive(Serialize)]
pub struct StatusVersion {
	pub name: String,
	pub protocol: i32,
}

#[derive(Serialize)]
pub struct StatusPlayers {
	pub max: usize,
	pub online: usize,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sample: Option<Vec<StatusPlayerSample>>,
}

#[derive(Serialize)]
pub struct StatusPlayerSample {
	pub name: String,
	pub id: Uuid,
}

#[derive(Serialize)]
pub struct StatusDescription<'a> {
	pub text: &'a str,
}

#[derive(Encode)]
#[repr(u8)]
pub enum Packet<'a> {
	#[encde(wire_tag = 0)]
	ReplyStatus(Json<StatusReply<'a>>),
	#[encde(wire_tag = 1)]
	Pong(i64),
}
