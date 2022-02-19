use super::super::varint::VarInt;
use super::super::wrappers::{std::*, util::encode_u8_slice};
use super::slot::Slot;
use encde::{Decode, Encode, Result as EResult};
use std::io::Write;

pub enum UnlockAction {
	Init { already_shown: PrefixedVec<PrefixedString>, new: PrefixedVec<PrefixedString> },
	Add(PrefixedVec<PrefixedString>),
	Remove(PrefixedVec<PrefixedString>),
}

#[derive(Encode)]
pub struct BookState {
	open: bool,
	filter_active: bool,
}

pub struct Recipe {
	id: PrefixedString,
	data: Type,
}

pub enum Type {
	Shapeless {
		group: PrefixedString,
		ingredients: PrefixedVec<Ingredient>,
		result: Slot,
	},
	Shaped {
		width: VarInt,
		height: VarInt,
		group: PrefixedString,
		/// width * height; row-major
		ingredients: Vec<Ingredient>,
		result: Slot,
	},
	ArmorDye,
	BookCloning,
	MapCloning,
	MapExtending,
	FireworkRocket,
	FireworkStar,
	FireworkStarFade,
	RepairItem,
	TippedArrow,
	BannerDuplicate,
	BannerAddPattern,
	ShieldDecoration,
	ShulkerBoxColoring,
	SuspiciousStew,
	Smelting(SmeltingRecipe),
	Blasting(SmeltingRecipe),
	Smoking(SmeltingRecipe),
	CampfireCooking(SmeltingRecipe),
	Stonecutting {
		group: PrefixedString,
		ingredient: Ingredient,
		result: Slot,
	},
	Smithing {
		base: Ingredient,
		addition: Ingredient,
		result: Slot,
	},
}

#[derive(Encode)]
pub struct SmeltingRecipe {
	group: PrefixedString,
	ingredient: Ingredient,
	result: Slot,
	experience: f32,
	cooking_time: VarInt,
}

pub type Ingredient = PrefixedVec<Slot>;

impl Encode for Recipe {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let identifier = match &self.data {
			Type::Shapeless { .. } => "crafting_shapeless",
			Type::Shaped { .. } => "crafting_shaped",
			Type::ArmorDye => "crafting_special_armordye",
			Type::BookCloning => "crafting_special_bookcloning",
			Type::MapCloning => "crafting_special_mapcloning",
			Type::MapExtending => "crafting_special_mapextending",
			Type::FireworkRocket => "crafting_special_firework_rocket",
			Type::FireworkStar => "crafting_special_firework_star",
			Type::FireworkStarFade => "crafting_special_firework_star_fade",
			Type::RepairItem => "crafting_special_repairitem",
			Type::TippedArrow => "crafting_special_tippedarrow",
			Type::BannerDuplicate => "crafting_special_bannerduplicate",
			Type::BannerAddPattern => "crafting_special_banneraddpattern",
			Type::ShieldDecoration => "crafting_special_shielddecoration",
			Type::ShulkerBoxColoring => "crafting_special_shulkerboxcoloring",
			Type::SuspiciousStew => "crafting_special_suspiciousstew",
			Type::Smelting(_) => "smelting",
			Type::Blasting(_) => "blasting",
			Type::Smoking(_) => "smoking",
			Type::CampfireCooking(_) => "campfire_cooking",
			Type::Stonecutting { .. } => "stonecutting",
			Type::Smithing { .. } => "smithing",
		};
		encode_u8_slice(writer, identifier.as_bytes())?;
		self.id.encode(writer)?;
		match &self.data {
			Type::Shapeless { group, ingredients, result } => {
				group.encode(writer)?;
				ingredients.encode(writer)?;
				result.encode(writer)?;
			}
			Type::Shaped { width, height, group, ingredients, result } => {
				width.encode(writer)?;
				height.encode(writer)?;
				group.encode(writer)?;
				// PANICS: this state is invalid and should not have occurred in the first place
				assert_eq!(ingredients.len(), usize::try_from(width.0).unwrap() * usize::try_from(height.0).unwrap());
				for item in ingredients.iter() {
					item.encode(writer)?;
				}
				result.encode(writer)?;
			}
			Type::ArmorDye => {}
			Type::BookCloning => {}
			Type::MapCloning => {}
			Type::MapExtending => {}
			Type::FireworkRocket => {}
			Type::FireworkStar => {}
			Type::FireworkStarFade => {}
			Type::RepairItem => {}
			Type::TippedArrow => {}
			Type::BannerDuplicate => {}
			Type::BannerAddPattern => {}
			Type::ShieldDecoration => {}
			Type::ShulkerBoxColoring => {}
			Type::SuspiciousStew => {}
			Type::Smelting(inner) => inner.encode(writer)?,
			Type::Blasting(inner) => inner.encode(writer)?,
			Type::Smoking(inner) => inner.encode(writer)?,
			Type::CampfireCooking(inner) => inner.encode(writer)?,
			Type::Stonecutting { group, ingredient, result } => {
				group.encode(writer)?;
				ingredient.encode(writer)?;
				result.encode(writer)?;
			}
			Type::Smithing { base, addition, result } => {
				base.encode(writer)?;
				addition.encode(writer)?;
				result.encode(writer)?;
			}
		};
		Ok(())
	}
}

#[derive(Encode, Decode)]
#[repr(u8)]
pub enum BookType {
	Crafting = 0,
	Furnace = 1,
	BlastFurnace = 2,
	Smoker = 3,
}
