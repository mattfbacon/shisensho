use super::super::varint::VarInt;
use super::super::wrappers::{nbt::NbtBlob, std::PrefixedOption};
use encde::{Decode, Encode};

pub type Slot = PrefixedOption<PresentSlot>;

#[derive(Encode, Decode)]
pub struct IndexedSlot {
	slot_index: i16,
	slot_data: Slot,
}

#[derive(Encode, Decode)]
pub struct PresentSlot {
	item_id: VarInt,
	count: i8,
	nbt_data: NbtBlob,
}
