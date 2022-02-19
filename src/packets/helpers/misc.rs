use super::game::slot::Slot;
use super::position::{F32Position, PackedPosition};
use super::varint::*;
use super::wrappers::std::*;
use super::wrappers::util::encode_u8_slice;
use encde::{Decode, Encode, Result as EResult};
use std::io::{Read, Write};

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum CardinalDirection {
	North = 2,
	South = 0,
	West = 1,
	East = 3,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum PaintingName {
	Kebab = 0,
	Aztec = 1,
	Alban = 2,
	Aztec2 = 3,
	Bomb = 4,
	Plant = 5,
	Wasteland = 6,
	Pool = 7,
	Courbet = 8,
	Sea = 9,
	Sunset = 10,
	CreeBet = 11,
	Wanderer = 12,
	Graham = 13,
	Match = 14,
	Bust = 15,
	Stage = 16,
	Void = 17,
	SkullAndRoses = 18,
	Wither = 19,
	Fighters = 20,
	Pointer = 21,
	PigScene = 22,
	BurningSkull = 23,
	Skeleton = 24,
	DonkeyKong = 25,
}

pub enum SculkDestination {
	Block(PackedPosition),
	Entity(VarInt),
}

impl Encode for SculkDestination {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		match self {
			Self::Block(position) => {
				encode_u8_slice(writer, "block".as_bytes())?;
				position.encode(writer)
			}
			Self::Entity(entity_id) => {
				encode_u8_slice(writer, "entity".as_bytes())?;
				entity_id.encode(writer)
			}
		}
	}
}

impl Decode for SculkDestination {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let discriminant = PrefixedString::decode(reader)?.0;
		match discriminant.as_str() {
			"block" => Ok(Self::Block(PackedPosition::decode(reader)?)),
			"entity" => Ok(Self::Entity(VarInt::decode(reader)?)),
			_ => Err(encde::Error::CustomStr("Invalid sculk destination; expected \"entity\" or \"block\"")),
		}
	}
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum AnimationType {
	SwingMainArm = 0,
	TakeDamage = 1,
	LeaveBed = 2,
	SwingOffhand = 3,
	CriticalEffect = 4,
	MagicCriticalEffect = 5,
}

/// Specific fields are TODO
#[derive(Encode, Decode)]
pub struct StatisticsEntry {
	category_id: VarInt,
	statistic_id: VarInt,
	value: VarInt,
}

pub enum DestroyStage {
	/// 0 to 9
	Breaking(u8),
	NotBreaking,
}

impl Encode for DestroyStage {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		match self {
			Self::Breaking(amount) => std::cmp::Ord::clamp(*amount, 0, 9).encode(writer),
			// any value outside of the 0..=9 range is acceptable
			Self::NotBreaking => u8::MAX.encode(writer),
		}
	}
}

impl Decode for DestroyStage {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		let raw = u8::decode(reader)?;
		Ok(match raw {
			amount @ 0..=9 => Self::Breaking(amount),
			_ => Self::NotBreaking,
		})
	}
}

#[derive(Encode, Decode)]
pub struct BlockAction {
	id: u8,
	param: u8,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum ServerDifficulty {
	Peaceful = 0,
	Easy = 1,
	Normal = 2,
	Hard = 3,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum GameMode {
	Survival = 0,
	Creative = 1,
	Adventure = 2,
	Spectator = 3,
}

#[derive(Encode, Decode)]
#[repr(i8)]
pub enum OptionalGameMode {
	None = -1,
	Survival = 0,
	Creative = 1,
	Adventure = 2,
	Spectator = 3,
}

#[derive(Encode)]
pub struct VillagerTrade {
	first_input_item: Slot,
	output_item: Slot,
	second_input_item: PrefixedOption<Slot>,
	trade_disabled: bool,
	num_uses: i32,
	max_uses: i32,
	xp_reward: i32,
	special_price: i32,
	price_fluctuation_multiplier: f32,
	demand: i32,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum PlayerHand {
	Main = 0,
	Off = 1,
}

#[derive(Encode)]
pub struct PlayerRotationTargetEntity {
	entity_id: VarInt,
	/// see documentation in UpdatePlayerRotation
	use_eyes: bool,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum BlockFace {
	/// -Y
	Bottom = 0,
	/// +Y
	Top = 1,
	/// -Z
	North = 2,
	/// +Z
	South = 3,
	/// -X
	West = 4,
	/// +X
	East = 5,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum GeneralAction {
	StartDigging = 0,
	CancelDigging = 1,
	FinishDigging = 2,
	DropItemStack = 3,
	DropItem = 4,
	/// Eat food, shoot arrow, use bucket, etc
	UseHeldItem = 5,
	SwapHands = 6,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum EntityAction {
	StartSneaking = 0,
	StopSneaking = 1,
	LeaveBed = 2,
	StartSprinting = 3,
	StopSprinting = 4,
	StartHorseJump = 5,
	StopHorseJump = 6,
	OpenHorseInventory = 7,
	StartElytraFlight = 8,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum CommandBlockMode {
	Sequence = 0,
	Auto = 1,
	Redstone = 2,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum StructureBlockAction {
	NoAction = 0,
	SaveStructure = 1,
	LoadStructure = 2,
	DetectSize = 3,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum StructureBlockUpdateType {
	Save = 0,
	Load = 1,
	Corner = 2,
	Data = 3,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum StructureBlockMirroring {
	None = 0,
	LeftRight = 1,
	FrontBack = 2,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum StructureBlockRotation {
	None = 0,
	Clockwise90 = 1,
	Clockwise180 = 2,
	Counterclockwise90 = 3,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum SoundCategory {
	Root = 0,
	Music = 1,
	Records = 2,
	Weather = 3,
	Blocks = 4,
	Hostile = 5,
	Neutral = 6,
	Players = 7,
	Ambient = 8,
	Voice = 9,
}

#[derive(Encode, Decode)]
#[repr(i32)]
pub enum EffectId {
	DispenserDispenses = 1000,
	DispenserFailsToDispense = 1001,
	DispenserShoots = 1002,
	EnderEyeLaunched = 1003,
	FireworkShot = 1004,
	IronDoorOpened = 1005,
	WoodenDoorOpened = 1006,
	WoodenTrapdoorOpened = 1007,
	FenceGateOpened = 1008,
	FireExtinguished = 1009,
	PlayRecord = 1010,
	IronDoorClosed = 1011,
	WoodenDoorClosed = 1012,
	WoodenTrapdoorClosed = 1013,
	FenceGateClosed = 1014,
	GhastWarns = 1015,
	GhastShoots = 1016,
	EnderDragonShoots = 1017,
	BlazeShoots = 1018,
	ZombieAttacksWoodDoor = 1019,
	ZombieAttacksIronDoor = 1020,
	ZombieBreaksWoodDoor = 1021,
	WitherBreaksBlock = 1022,
	WitherSpawned = 1023,
	WitherShoots = 1024,
	BatTakesOff = 1025,
	ZombieInfects = 1026,
	ZombieVillagerConverted = 1027,
	EnderDragonDeath = 1028,
	AnvilDestroyed = 1029,
	AnvilUsed = 1030,
	AnvilLanded = 1031,
	PortalTravel = 1032,
	ChorusFlowerGrown = 1033,
	ChorusFlowerDied = 1034,
	BrewingStandBrewed = 1035,
	IronTrapdoorOpened = 1036,
	IronTrapdoorClosed = 1037,
	EndPortalCreatedInOverworld = 1038,
	PhantomBites = 1039,
	ZombieConvertsToDrowned = 1040,
	HuskConvertsToZombieByDrowning = 1041,
	GrindstoneUsed = 1042,
	BookPageTurned = 1043,
	ComposterComposts = 1500,
	LavaConvertsBlock = 1501,
	RedstoneTorchBurnsOut = 1502,
	EnderEyePlaced = 1503,
	Spawn10SmokeParticles = 2000,
	BlockBreak = 2001,
	SplashPotion = 2002,
	EyeOfEnderBreak = 2003,
	MobSpawnParticleEffect = 2004,
	BonemealParticles = 2005,
	DragonBreath = 2006,
	InstantSplashPotion = 2007,
	EnderDragonDestroysBlock = 2008,
	WetSpongeVaporizesInNether = 2009,
	EndGatewaySpawn = 3000,
	EnderDragonGrowl = 3001,
	ElectricSpark = 3002,
	CopperApplyWax = 3003,
	CopperRemoveWax = 3004,
	CopperScrapeOxidation = 3005,
}

#[derive(Encode)]
pub struct TagGroup {
	group_name: PrefixedString,
	tags: PrefixedVec<TaggedIds>,
}

#[derive(Encode)]
pub struct TaggedIds {
	tag_name: PrefixedString,
	ids: PrefixedVec<VarInt>,
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum InteractionType {
	#[encde(wire_tag = 0)]
	Interact { hand: PlayerHand },
	#[encde(wire_tag = 1)]
	Attack,
	#[encde(wire_tag = 2)]
	InteractAt { position: F32Position, hand: PlayerHand },
}

/// TODO custom type
pub type PotionId = VarInt;
