use super::biome::BiomeRegistry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Codec {
	#[serde(rename = "minecraft:dimension_type")]
	dimension_types: TypeRegistry,
	#[serde(rename = "minecraft:worldgen/biome")]
	biomes: BiomeRegistry,
}

pub struct TypeRegistry(Vec<TypeEntry>);

#[derive(Serialize)]
struct TypeRegistryWireSer<'a> {
	#[serde(rename = "type")]
	ty: &'a str,
	value: &'a [TypeEntry],
}
#[derive(Deserialize)]
struct TypeRegistryWireDe<'a> {
	#[serde(rename = "type")]
	ty: &'a str,
	value: Vec<TypeEntry>,
}
impl Serialize for TypeRegistry {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		TypeRegistryWireSer {
			ty: "minecraft:dimension_type",
			value: &self.0,
		}
		.serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for TypeRegistry {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		Ok(Self(TypeRegistryWireDe::deserialize(deserializer)?.value))
	}
}

#[derive(Serialize, Deserialize)]
pub struct TypeEntry {
	name: String,
	id: i32,
	element: Type,
}

#[derive(Serialize, Deserialize)]
pub struct Type {
	piglin_safe: bool,
	natural: bool,
	ambient_light: f32,
	fixed_time: Option<i64>,
	#[serde(rename = "infiniburn")]
	infinite_burn: String,
	respawn_anchor_works: bool,
	has_skylight: bool,
	bed_works: bool,
	#[serde(rename = "effects")]
	dimension_name: String,
	has_raids: bool,
	min_y: i32,
	height: i32,
	logical_height: i32,
	coordinate_scale: f32,
	#[serde(rename = "ultrawarm")]
	ultra_warm: bool,
	has_ceiling: bool,
}
