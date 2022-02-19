use super::super::varint::VarInt;
use super::super::wrappers::std::PrefixedString;
use super::super::wrappers::util::encode_u8_slice;
use super::chat::Chat;
use encde::{Encode, Result as EResult};
use std::io::Write;

pub enum Position {
	List,
	Sidebar,
	BelowName,
	TeamSidebar(u8),
}

impl Encode for Position {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let encoded: u8 = match self {
			Self::List => 0,
			Self::Sidebar => 1,
			Self::BelowName => 2,
			Self::TeamSidebar(team) => 3 + team,
		};
		encoded.encode(writer)
	}
}

#[derive(Encode)]
#[repr(u8)]
pub enum ObjectiveUpdate {
	#[encde(wire_tag = 0)]
	Create(ObjectiveData),
	#[encde(wire_tag = 1)]
	Remove,
	#[encde(wire_tag = 2)]
	Update(ObjectiveData),
}

#[derive(Encode)]
pub struct ObjectiveData {
	value: Chat,
	ty: ObjectiveType,
}

#[derive(Encode)]
#[repr(u8)]
pub enum ObjectiveType {
	Integer = 0,
	Hearts = 1,
}

// TODO finish
/*
#[derive(Encode)]
#[repr(u8)]
pub enum TeamUpdate {
	#[encde(wire_tag = 0)]
	CreateTeam {
		display_name: Chat,
		/// Bit flags (TODO custom type)
		/// 1 = allow friendly fire
		/// 2 = can see invisible players on the same team
		friendly_flags: u8,
		name_tag_visibility: NameTagVisibility,
		collision_rule: CollisionRule,
	},
	#[encde(wire_tag = 1)]
	RemoveTeam,
	#[encde(wire_tag = 2)]
	UpdateTeam,
	#[encde(wire_tag = 3)]
	AddMembers,
	#[encde(wire_tag = 4)]
	RemoveMembers,
}
*/

pub enum NameTagVisibility {
	Always,
	HideForOtherTeams,
	HideForOwnTeam,
	Never,
}

impl Encode for NameTagVisibility {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let s = match self {
			Self::Always => "always",
			Self::HideForOtherTeams => "hideForOtherTeams",
			Self::HideForOwnTeam => "hideForOwnTeam",
			Self::Never => "never",
		};
		encode_u8_slice(writer, s.as_bytes())
	}
}

pub enum CollisionRule {
	Always,
	PushOtherTeams,
	PushOwnTeam,
	Never,
}

impl Encode for CollisionRule {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let s = match self {
			Self::Always => "always",
			Self::PushOtherTeams => "pushOtherTeams",
			Self::PushOwnTeam => "pushOwnTeam",
			Self::Never => "never",
		};
		encode_u8_slice(writer, s.as_bytes())
	}
}

#[derive(Encode)]
#[repr(u8)]
pub enum ScoreUpdate {
	#[encde(wire_tag = 0)]
	CreateUpdate { objective_name: PrefixedString, value: VarInt },
	#[encde(wire_tag = 1)]
	Remove { objective_name: PrefixedString },
}
