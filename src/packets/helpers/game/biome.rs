use serde::{Deserialize, Serialize};

pub struct BiomeRegistry(Vec<BiomeRegistryEntry>);

#[derive(Deserialize)]
struct BiomeRegistryWireDe<'a> {
	#[serde(rename = "type")]
	ty: &'a str,
	value: Vec<BiomeRegistryEntry>,
}
#[derive(Serialize)]
struct BiomeRegistryWireSer<'a> {
	#[serde(rename = "type")]
	ty: &'a str,
	value: &'a [BiomeRegistryEntry],
}
impl Serialize for BiomeRegistry {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		BiomeRegistryWireSer {
			ty: "minecraft:worldgen/biome",
			value: &self.0,
		}
		.serialize(serializer)
	}
}
impl<'de> Deserialize<'de> for BiomeRegistry {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		Ok(Self(BiomeRegistryWireDe::deserialize(deserializer)?.value))
	}
}

#[derive(Serialize, Deserialize)]
pub struct BiomeRegistryEntry {
	name: String,
	id: i32,
	element: BiomeProperties,
}

#[derive(Serialize, Deserialize)]
pub struct BiomeProperties {
	precipitation: String,
	depth: f32,
	temperature: f32,
	scale: f32,
	downfall: f32,
	category: String,
	temperature_modifier: Option<String>,
	effects: BiomeEffects,
	particle: Option<BiomeParticle>,
}

#[derive(Serialize, Deserialize)]
pub struct BiomeEffects {
	sky_color: i32,
	water_fog_color: i32,
	fog_color: i32,
	water_color: i32,
	foliage_color: Option<i32>,
	grass_color: Option<i32>,
	grass_color_modifier: Option<String>,
	music: Option<BiomeMusic>,
	ambient_sound: Option<String>,
	additions_sound: Option<BiomeAdditionsSound>,
	mood_sound: Option<BiomeMoodSound>,
}

#[derive(Serialize, Deserialize)]
pub struct BiomeMusic {
	replace_current_music: bool,
	sound: String,
	max_delay: i32,
	min_delay: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BiomeAdditionsSound {
	sound: String,
	tick_chance: f64,
}

#[derive(Serialize, Deserialize)]
pub struct BiomeMoodSound {
	sound: String,
	tick_delay: i32,
	offset: f64,
	block_search_extent: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BiomeParticle {
	probability: f32,
	options: BiomeParticleOptions,
}

#[derive(Serialize, Deserialize)]
pub struct BiomeParticleOptions {
	#[serde(rename = "type")]
	ty: String,
}
