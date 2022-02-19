use super::super::misc::GameMode;
use super::super::varint::VarInt;
use super::super::wrappers::{std::*, uuid::Uuid};
use super::chat::Chat;
use encde::Encode;

#[derive(Encode)]
pub struct AddPlayer {
	player_uuid: Uuid,
	/// Usually one item named "textures" with the profile, skin, and cape data from the Mojang API as Base-64 JSON
	properties: PrefixedVec<AddPlayerProperty>,
	gamemode: GameMode,
	ping: VarInt,
	display_name: PrefixedOption<Chat>,
}

#[derive(Encode)]
pub struct AddPlayerProperty {
	name: PrefixedString,
	value: PrefixedString,
	signature: PrefixedOption<PrefixedBytes>,
}

#[derive(Encode)]
pub struct UpdateGamemode {
	player_uuid: Uuid,
	new_gamemode: GameMode,
}

#[derive(Encode)]
pub struct UpdateLatency {
	player_uuid: Uuid,
	ping: VarInt,
}

#[derive(Encode)]
pub struct UpdateDisplayName {
	player_uuid: Uuid,
	display_name: PrefixedOption<Chat>,
}

#[derive(Encode)]
pub struct RemovePlayer {
	player_uuid: Uuid,
}
