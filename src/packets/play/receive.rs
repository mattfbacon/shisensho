use crate::packets::helpers;
use encde::Decode;
use helpers::game::*;
use helpers::misc;
use helpers::position as pos;
use helpers::rotation as rot;
use helpers::varint::*;
use helpers::wrappers::std::*;
use helpers::wrappers::uuid::Uuid;

#[derive(Decode)]
pub struct ConfirmTeleport {
	teleport_id: VarInt,
}

#[derive(Decode)]
pub struct QueryBlockNbt {
	transaction_id: VarInt,
	location: pos::PackedPosition,
}

#[derive(Decode)]
pub struct SetDifficulty(misc::ServerDifficulty);

#[derive(Decode)]
pub struct SendChatMessage(PrefixedString);

#[derive(Decode)]
#[repr(u8)]
pub enum RequestMisc {
	PerformRespawn = 0,
	RequestStats = 1,
}

#[derive(Decode)]
pub struct UpdateClientSettings {
	locale: PrefixedString,
	render_distance: i8,
	chat_mode: chat::ClientChatMode,
	chat_colors: bool,
	/// Bit flags (TODO custom type)
	/// 1 = cape
	/// 2 = jacket
	/// 4 = left sleeve
	/// 8 = right sleeve
	/// 16 = left pants leg
	/// 32 = right pants leg
	/// 64 = hat
	skin_parts: u8,
	right_handed: bool,
	/// Currently unused
	text_filtering: bool,
	show_in_server_listing: bool,
}

#[derive(Decode)]
pub struct TriggerTabComplete {
	transaction_id: VarInt,
	prompt: PrefixedString,
}

#[derive(Decode)]
pub struct ClickWindowButton {
	window_id: i8,
	button_id: i8,
}

#[derive(Decode)]
pub struct ClickWindowSlot {
	window_id: u8,
	state_id: VarInt,
	// TODO logic, enums, custom types
	clicked_slot_index: i16,
	button: i8,
	mode: VarInt,
	updated_slots: PrefixedVec<slot::IndexedSlot>,
	clicked_slot_data: slot::Slot,
}

#[derive(Decode)]
pub struct CloseWindow {
	window_id: u8,
}

// TODO something with DecodeSized
// #[derive(Decode)]
pub struct PluginMessage {
	channel: PrefixedString,
	data: UnprefixedBytes,
}

#[derive(Decode)]
pub struct EditBook {
	hand: misc::PlayerHand,
	pages: PrefixedVec<PrefixedString>,
	title: PrefixedOption<PrefixedString>,
}

#[derive(Decode)]
pub struct QueryEntityNbt {
	transaction_id: VarInt,
	entity_id: VarInt,
}

#[derive(Decode)]
pub struct InteractWithEntity {
	target_id: VarInt,
	interaction_type: misc::InteractionType,
	sneaking: bool,
}

#[derive(Decode)]
pub struct GenerateStructure {
	location: pos::PackedPosition,
	levels: VarInt,
	keep_jigsaws: bool,
}

#[derive(Decode)]
pub struct KeepAlive(i64);

#[derive(Decode)]
pub struct SetDifficultyLocked {
	locked: bool,
}

#[derive(Decode)]
pub struct MovePosition {
	new_position: pos::F64Position,
	on_ground: bool,
}

#[derive(Decode)]
pub struct MoveRotation {
	new_rotation: rot::F32Rotation,
	on_ground: bool,
}

#[derive(Decode)]
pub struct MovePosRot {
	new_position: pos::F64Position,
	new_rotation: rot::F32Rotation,
	on_ground: bool,
}

#[derive(Decode)]
pub struct MoveStationary {
	on_ground: bool,
}

#[derive(Decode)]
pub struct MoveVehicle {
	new_position: pos::F64Position,
	new_rotation: rot::F32Rotation,
}

#[derive(Decode)]
pub struct SteerBoat {
	left_paddle: bool,
	right_paddle: bool,
}

#[derive(Decode)]
pub struct PickItemFromInventory {
	from_slot: VarInt,
}

#[derive(Decode)]
pub struct SelectCraftRecipe {
	window_id: i8,
	recipe: PrefixedString,
	make_all: bool,
}

#[derive(Decode)]
pub struct UpdateFlying {
	/// Matches flags in send::UpdatePlayerAbilities
	/// Will have bit 1 (mask = 0x2) set or unset based on whether flying started or stopped
	flags: u8,
}

/// As the name suggests, this is very general. The `block` and `face` fields exist due to it being used for digging blocks, however it's also used for various other actions, in which case `block` and `face` can be ignored.
#[derive(Decode)]
pub struct TakeGeneralAction {
	action: misc::GeneralAction,
	block: pos::PackedPosition,
	face: misc::BlockFace,
}

#[derive(Decode)]
pub struct TakeEntityAction {
	/// This can probably be ignored since we know the Player ID
	player_id: VarInt,
	action: misc::EntityAction,
	/// Only meaningful with "Start Horse Jump" action
	jump_boost: VarInt,
}

#[derive(Decode)]
pub struct SteerVehicle {
	sideways: f32,
	forward: f32,
	/// Bit flags (TODO custom type)
	/// 1 = Jump
	/// 2 = Unmount
	flags: u8,
}

#[derive(Decode)]
pub struct Pong(i32);

#[derive(Decode)]
pub struct SetRecipeBookState {
	book_id: recipes::BookType,
	book_open: bool,
	filter_active: bool,
}

#[derive(Decode)]
pub struct SetDisplayedRecipe {
	recipe_id: PrefixedString,
}

/// Sent as the player types
#[derive(Decode)]
pub struct UpdateCustomItemName {
	item_name: PrefixedString,
}

#[derive(Decode)]
#[repr(u8)]
pub enum ResourcePackStatus {
	SuccessfullyLoaded = 0,
	Declined = 1,
	DownloadFailed = 2,
	Accepted = 3,
}

#[derive(Decode)]
pub enum UpdateAdvancementMenu {
	SelectTab(PrefixedString),
	CloseWindow,
}

#[derive(Decode)]
pub struct SelectTrade {
	selected_slot: VarInt,
}

#[derive(Decode)]
pub struct SetBeaconEffect {
	primary_effect: misc::PotionId,
	secondary_effect: misc::PotionId,
}

#[derive(Decode)]
pub struct ChangeHeldItem(i16);

#[derive(Decode)]
pub struct UpdateCommandBlock {
	location: pos::PackedPosition,
	command: PrefixedString,
	mode: misc::CommandBlockMode,
	/// Bit flags (TODO custom type)
	/// 1 = Track output
	/// 2 = Conditional
	/// 4 = Automatic
	flags: u8,
}

#[derive(Decode)]
pub struct UpdateCommandBlockMinecart {
	entity_id: VarInt,
	command: PrefixedString,
	track_output: bool,
}

/// Creative only
#[derive(Decode)]
pub struct CheatInventorySlot(slot::IndexedSlot);

#[derive(Decode)]
pub struct UpdateJigsawBlock {
	location: pos::PackedPosition,
	name: PrefixedString,
	target: PrefixedString,
	pool: PrefixedString,
	final_state: PrefixedString,
	joint_type: PrefixedString,
}

#[derive(Decode)]
pub struct UpdateStructureBlock {
	location: pos::PackedPosition,
	action: misc::StructureBlockAction,
	mode: misc::StructureBlockUpdateType,
	name: PrefixedString,
	offset: pos::UnpackedPosition<i8>,
	size: pos::UnpackedPosition<i8>,
	mirror: misc::StructureBlockMirroring,
	rotation: misc::StructureBlockRotation,
	metadata: PrefixedString,
	integrity: f32,
	seed: VarLong,
	/// Bit flags (TODO custom type)
	/// 1 = Ignore entities
	/// 2 = Show air
	/// 4 = Show bounding box
	flags: u8,
}

#[derive(Decode)]
pub struct UpdateSign {
	location: pos::PackedPosition,
	line1: PrefixedString,
	line2: PrefixedString,
	line3: PrefixedString,
	line4: PrefixedString,
}

#[derive(Decode)]
pub struct TriggerArmAnimation(misc::PlayerHand);

#[derive(Decode)]
pub struct SpectateEntity(Uuid);

#[derive(Decode)]
pub struct PlaceBlock {
	hand: misc::PlayerHand,
	location: pos::PackedPosition,
	face: misc::BlockFace,
	cursor_position_within_block: pos::F32Position,
	head_inside_block: bool,
}

#[derive(Decode)]
pub struct UseItem(misc::PlayerHand);

#[derive(Decode)]
#[repr(u8)]
pub enum Packet {
	#[encde(wire_tag = 0x00)]
	ConfirmTeleport(ConfirmTeleport),
	#[encde(wire_tag = 0x01)]
	QueryBlockNbt(QueryBlockNbt),
	#[encde(wire_tag = 0x02)]
	SetDifficulty(misc::ServerDifficulty),
	#[encde(wire_tag = 0x03)]
	SendChatMessage(SendChatMessage),
	#[encde(wire_tag = 0x04)]
	RequestMisc(RequestMisc),
	#[encde(wire_tag = 0x05)]
	UpdateClientSettings(UpdateClientSettings),
	#[encde(wire_tag = 0x06)]
	TriggerTabComplete(TriggerTabComplete),
	#[encde(wire_tag = 0x07)]
	ClickWindowButton(ClickWindowButton),
	#[encde(wire_tag = 0x08)]
	ClickWindowSlot(ClickWindowSlot),
	#[encde(wire_tag = 0x09)]
	CloseWindow(CloseWindow),
	// #[encde(wire_tag = 0x0a)]
	// PluginMessage(PluginMessage),
	#[encde(wire_tag = 0x0b)]
	EditBook(EditBook),
	#[encde(wire_tag = 0x0c)]
	QueryEntityNbt(QueryEntityNbt),
	#[encde(wire_tag = 0x0d)]
	InteractWithEntity(InteractWithEntity),
	#[encde(wire_tag = 0x0e)]
	GenerateStructure(GenerateStructure),
	#[encde(wire_tag = 0x0f)]
	KeepAlive(KeepAlive),
	#[encde(wire_tag = 0x10)]
	SetDifficultyLocked(SetDifficultyLocked),
	#[encde(wire_tag = 0x11)]
	MovePosition(MovePosition),
	#[encde(wire_tag = 0x12)]
	MovePosRot(MovePosRot),
	#[encde(wire_tag = 0x13)]
	MoveRotation(MoveRotation),
	#[encde(wire_tag = 0x14)]
	MoveStationary(MoveStationary),
	#[encde(wire_tag = 0x15)]
	MoveVehicle(MoveVehicle),
	#[encde(wire_tag = 0x16)]
	SteerBoat(SteerBoat),
	#[encde(wire_tag = 0x17)]
	PickItemFromInventory(PickItemFromInventory),
	#[encde(wire_tag = 0x18)]
	SelectCraftRecipe(SelectCraftRecipe),
	#[encde(wire_tag = 0x19)]
	UpdateFlying(UpdateFlying),
	#[encde(wire_tag = 0x1a)]
	TakeGeneralAction(TakeGeneralAction),
	#[encde(wire_tag = 0x1b)]
	TakeEntityAction(TakeEntityAction),
	#[encde(wire_tag = 0x1c)]
	SteerVehicle(SteerVehicle),
	#[encde(wire_tag = 0x1d)]
	Pong(Pong),
	#[encde(wire_tag = 0x1e)]
	SetRecipeBookState(SetRecipeBookState),
	#[encde(wire_tag = 0x1f)]
	SetDisplayedRecipe(SetDisplayedRecipe),
	#[encde(wire_tag = 0x20)]
	UpdateCustomItemName(UpdateCustomItemName),
	#[encde(wire_tag = 0x21)]
	ResourcePackStatus(ResourcePackStatus),
	#[encde(wire_tag = 0x22)]
	UpdateAdvancementMenu(UpdateAdvancementMenu),
	#[encde(wire_tag = 0x23)]
	SelectTrade(SelectTrade),
	#[encde(wire_tag = 0x24)]
	SetBeaconEffect(SetBeaconEffect),
	#[encde(wire_tag = 0x25)]
	ChangeHeldItem(ChangeHeldItem),
	#[encde(wire_tag = 0x26)]
	UpdateCommandBlock(UpdateCommandBlock),
	#[encde(wire_tag = 0x27)]
	UpdateCommandBlockMinecart(UpdateCommandBlockMinecart),
	#[encde(wire_tag = 0x28)]
	CheatInventorySlot(CheatInventorySlot),
	#[encde(wire_tag = 0x29)]
	UpdateJigsawBlock(UpdateJigsawBlock),
	#[encde(wire_tag = 0x2a)]
	UpdateStructureBlock(UpdateStructureBlock),
	#[encde(wire_tag = 0x2b)]
	UpdateSign(UpdateSign),
	#[encde(wire_tag = 0x2c)]
	TriggerArmAnimation(TriggerArmAnimation),
	#[encde(wire_tag = 0x2d)]
	SpectateEntity(SpectateEntity),
	#[encde(wire_tag = 0x2e)]
	PlaceBlock(PlaceBlock),
	#[encde(wire_tag = 0x2f)]
	UseItem(UseItem),
}
