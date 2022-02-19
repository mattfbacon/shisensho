use super::super::wrappers::{std::*, uuid::Uuid};
use super::entity;
use super::slot::Slot;
use encde::Encode;

// TODO implement
// #[derive(Encode)]
pub struct Metadata(());

#[derive(Encode)]
pub struct Property {
	key: PrefixedString,
	value: f64,
	/// These must be applied in order of their type: Add, then AddPercent, then Multiply
	modifiers: PrefixedVec<entity::PropertyModifier>,
}

#[derive(Encode)]
pub struct PropertyModifier {
	uuid: Uuid,
	amount: f64,
	operation: PropertyModifierType,
}

#[derive(Encode)]
#[repr(u8)]
pub enum PropertyModifierType {
	/// value += amount
	Add = 0,
	/// value += value * (amount / 100)
	AddPercent = 1,
	/// value *= amount
	Multiply = 2,
}

#[derive(Encode)]
pub struct EquipmentEntry {
	slot: EquipmentSlot,
	item: Slot,
}

#[derive(Encode)]
#[repr(u8)]
pub enum EquipmentSlot {
	MainHand = 0,
	OffHand = 1,
	Boots = 2,
	Leggings = 3,
	Chestplate = 4,
	Helmet = 5,
}

pub type Velocity = super::super::position::UnpackedPosition<i16>;
