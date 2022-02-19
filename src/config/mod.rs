use figment::{
	providers::{Env, Format, Toml},
	Error, Figment,
};
use serde::Deserialize;
use std::path::PathBuf;

fn default_port() -> u16 {
	25565
}

#[derive(Deserialize)]
pub struct Config {
	pub address: std::net::IpAddr,
	#[serde(default = "default_port")]
	pub port: u16,
	pub logging: Vec<LoggingConfig>,
	#[serde(default)]
	pub listing: ListingConfig,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
	pub level: log::LevelFilter,
	#[serde(flatten)]
	pub sink: LoggingSink,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum LoggingSink {
	File { file: PathBuf },
	Console { console: ConsoleType },
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ConsoleType {
	Stdout,
	Stderr,
}
impl From<ConsoleType> for log4rs::append::console::Target {
	fn from(ty: ConsoleType) -> Self {
		match ty {
			ConsoleType::Stdout => Self::Stdout,
			ConsoleType::Stderr => Self::Stderr,
		}
	}
}

fn default_motd() -> String {
	"Running RustMC!".to_owned()
}

fn default_icon() -> Option<String> {
	None
}

fn deserialize_server_icon<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
	use serde::de::Error;
	let path = <Option<PathBuf>>::deserialize(deserializer)?;
	match path {
		None => Ok(None),
		Some(path) => {
			let mut writer = base64::write::EncoderStringWriter::from("data:image/png;base64,".to_owned(), base64::STANDARD);
			let mut file = std::fs::File::open(path).map_err(|err| D::Error::custom(err.to_string()))?;
			std::io::copy(&mut file, &mut writer).map_err(|err| D::Error::custom(err.to_string()))?;
			Ok(Some(writer.into_inner()))
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct ListingConfig {
	/// stored as Base-64-encoded PNG data ("data:image/png;base64,<data>")
	#[serde(deserialize_with = "deserialize_server_icon", default = "default_icon")]
	pub icon: Option<String>,
	#[serde(default = "default_motd")]
	pub motd: String,
}

impl Default for ListingConfig {
	fn default() -> Self {
		Self { icon: default_icon(), motd: default_motd() }
	}
}

pub fn load() -> Result<Config, Error> {
	let raw = Figment::new().merge(Toml::file("server.toml")).merge(Env::prefixed("RUSTMC_"));
	raw.extract()
}
