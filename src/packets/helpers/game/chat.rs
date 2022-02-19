use super::super::wrappers::json::Json;
use super::super::wrappers::std::{PrefixedOption, PrefixedString};
use encde::{Decode, Encode};
use serde::Serialize;

pub type Chat = Json<Content>;

#[derive(Serialize, Default)]
pub struct Content {
	pub text: String,
	#[serde(flatten)]
	pub style: Style,
	#[serde(flatten)]
	pub actions: Actions,
	pub extra: Option<Vec<Content>>,
}

#[derive(Serialize, Default)]
pub struct Style {
	pub bold: Option<bool>,
	pub italic: Option<bool>,
	pub strikethrough: Option<bool>,
	pub obfuscated: Option<bool>,
	pub font: Option<Font>,
	pub color: Option<Color>,
}

#[derive(Serialize)]
pub enum Font {
	#[serde(rename = "minecraft:uniform")]
	Uniform,
	#[serde(rename = "minecraft:alt")]
	Alt,
	#[serde(rename = "minecraft:default")]
	Default,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
	Black,
	DarkBlue,
	DarkGreen,
	DarkAqua,
	DarkRed,
	DarkPurple,
	Gold,
	Gray,
	DarkGray,
	Blue,
	Green,
	Aqua,
	Red,
	LightPurple,
	Yellow,
	White,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Actions {
	pub insertion: Option<String>,
	pub click_event: Option<ClickEvent>,
	pub hover_event: Option<HoverEvent>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action", content = "value")]
pub enum ClickEvent {
	OpenUrl(String),
	RunCommand(String),
	SuggestCommand(String),
	ChangePage(u64),
	CopyToClipboard(String),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action", content = "value")]
pub enum HoverEvent {
	/// TODO more specific types
	ShowText(serde_json::Value),
	/// TODO more specific types
	ShowItem(serde_json::Value),
	/// TODO more specific types
	ShowEntity(serde_json::Value),
}

#[derive(Encode, Decode)]
#[repr(i8)]
pub enum Position {
	Chat = 0,
	System = 1,
	GameInfo = 2,
}

#[derive(Encode, Decode)]
#[repr(i8)]
pub enum ClientChatMode {
	Enabled = 0,
	CommandsOnly = 1,
	Hidden = 2,
}

#[derive(Encode)]
pub struct TabCompletion {
	text: PrefixedString,
	tooltip: PrefixedOption<Chat>,
}
