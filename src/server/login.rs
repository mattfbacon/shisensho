use super::Client;
use crate::packets::cipher::{Cipher, CipherWrapper};
use crate::packets::helpers::wrappers::std::{PrefixedArray, PrefixedBorrowedBytes, PrefixedBytes, PrefixedString};
use crate::packets::helpers::wrappers::uuid::Uuid as UuidWrapper;
use crate::packets::login::receive::{self, Packet as Receive};
use crate::packets::login::send::Packet as Send;
use log::trace;
use serde::{Deserialize, Deserializer};
use sha::utils::{Digest, DigestExt};

fn deserialize_skin_texture<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
	use serde::de::{self, Error};
	use std::fmt;
	#[derive(Deserialize)]
	struct Property {
		name: String,
		value: String,
		signature: Option<String>,
	}

	struct Visitor;
	impl<'de> de::Visitor<'de> for Visitor {
		type Value = Vec<u8>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("A sequence of session properties including the `textures` property")
		}
		fn visit_seq<S: de::SeqAccess<'de>>(self, mut access: S) -> Result<Self::Value, S::Error>
		where
			S::Error: Error,
		{
			while let Some(property) = access.next_element::<Property>()? {
				if property.name == "textures" {
					// TODO check signature; I can't find the Yggdrasil public key anywhere
					return base64::decode(property.value).map_err(|err| S::Error::custom(err.to_string()));
				}
			}
			Err(S::Error::custom("No `textures` property present"))
		}
	}
	deserializer.deserialize_seq(Visitor)
}

#[derive(Deserialize)]
pub struct SessionResponse {
	#[serde(rename = "id")]
	pub uuid: uuid::Uuid,
	#[serde(rename = "name")]
	pub username: String,
	#[serde(deserialize_with = "deserialize_skin_texture")]
	#[serde(rename = "properties")]
	pub skin_texture: Vec<u8>,
}

fn format_minecraft_sha1(mut data: &mut [u8]) -> String {
	use std::fmt::Write;
	let negative = (data[0] & 0x80) == 0x80;
	let mut ret = String::with_capacity((data.len() * 2) + 1);
	if negative {
		ret += "-";
		// two's complement
		let mut carry = true;
		for byte in data.iter_mut().rev() {
			*byte = !*byte;
			if carry {
				// destructuring assignments are unstable
				let result = byte.overflowing_add(1);
				*byte = result.0;
				carry = result.1;
			}
		}
	}
	let first_nonzero = data.iter().position(|&x| x != 0).unwrap_or(data.len());
	data = &mut data[first_nonzero..];
	// PANICS: this is guaranteed not to return Err
	write!(ret, "{:x}", data[0]).unwrap();
	for byte in data.iter().skip(1) {
		// PANICS: ditto
		write!(ret, "{:02x}", byte).unwrap();
	}
	ret
}

#[cfg(test)]
mod test {
	use sha::utils::{Digest, DigestExt};
	pub fn format_sha(plain: &str) -> String {
		super::format_minecraft_sha1(&mut sha::sha1::Sha1::default().digest(plain.as_bytes()).to_bytes())
	}
	#[test]
	pub fn sha1_formatting() {
		assert_eq!("4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48", format_sha("Notch"));
		assert_eq!("-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1", format_sha("jeb_"));
		assert_eq!("88e16a1019277b15d58faf0541e11910eb756f6", format_sha("simon"));
	}
}

fn rsa_private_decrypt(key: &openssl::rsa::RsaRef<openssl::pkey::Private>, data: &[u8]) -> anyhow::Result<Vec<u8>> {
	let mut buf = vec![0u8; key.size() as usize];
	let size = key.private_decrypt(data, &mut buf, openssl::rsa::Padding::PKCS1)?;
	buf.truncate(size);
	Ok(buf)
}

impl Client {
	fn receive_login_start(&mut self) -> anyhow::Result<String> {
		match self.receive_packet()? {
			None => anyhow::bail!("Client closed connection"),
			Some(Receive::LoginStart { username }) => Ok(username.0),
			Some(other) => anyhow::bail!("Expected Login Start packet but received {:?}", other),
		}
	}
	fn receive_encryption_response(&mut self) -> anyhow::Result<receive::Encryption> {
		match self.receive_packet()? {
			None => anyhow::bail!("Client closed connection"),
			Some(Receive::Encryption(info)) => Ok(info),
			Some(other) => anyhow::bail!("Expected Encryption Response packet but received {:?}", other),
		}
	}
	fn request_encryption(&mut self) -> anyhow::Result<[u8; 4]> {
		trace!("Requesting encryption");
		let verify_token = rand::random();
		self.send_packet(&Send::Encryption {
			server_id: PrefixedString(String::new()),
			public_key: PrefixedBorrowedBytes(&self.global_state.rsa_public_der),
			verify_token: PrefixedArray(verify_token),
		})?;
		Ok(verify_token)
	}
	fn receive_shared_secret(&mut self, verify_token: [u8; 4]) -> anyhow::Result<Vec<u8>> {
		let receive::Encryption {
			shared_secret: PrefixedBytes(encrypted_shared_secret),
			verify_token: PrefixedBytes(encrypted_verify_token),
		} = self.receive_encryption_response()?;
		trace!("Received encryption response");
		let decrypted_verify_token = rsa_private_decrypt(&self.global_state.rsa_key, &encrypted_verify_token)?;
		anyhow::ensure!(verify_token == decrypted_verify_token.as_slice(), "Verify token does not match");
		Ok(rsa_private_decrypt(&self.global_state.rsa_key, &encrypted_shared_secret)?)
	}
	fn make_cipher(shared_secret: &[u8]) -> anyhow::Result<Cipher> {
		use cfb8::cipher::NewCipher;
		Ok(Cipher::new_from_slices(shared_secret, shared_secret)?)
	}
	fn get_session(&mut self, username: String, shared_secret: &[u8]) -> anyhow::Result<SessionResponse> {
		let mut auth_hash = sha::sha1::Sha1::default().digest(b"").digest(shared_secret).digest(&self.global_state.rsa_public_der).to_bytes();
		let auth_hash = format_minecraft_sha1(&mut auth_hash);
		trace!("Making request to session server");
		let response = reqwest::blocking::get(reqwest::Url::parse_with_params(
			"https://sessionserver.mojang.com/session/minecraft/hasJoined",
			&[("username", username.as_str()), ("serverId", auth_hash.as_str())],
		)?)?
		.error_for_status()?;
		// This is a bit of a kludge, but we have to handle the No Content response somehow
		if response.status() == reqwest::StatusCode::NO_CONTENT {
			trace!("Session server returned No Content; using other endpoints to synthesize the data");
			#[derive(Deserialize)]
			struct UuidForUsernameResponse {
				// there is also the "name" field but we can ignore it
				#[serde(rename = "id")]
				uuid: uuid::Uuid,
			}
			let uuid: UuidForUsernameResponse = reqwest::blocking::get(format!("https://api.mojang.com/users/profiles/minecraft/{}", username))?.error_for_status()?.json()?;
			let uuid = uuid.uuid;
			Ok(reqwest::blocking::get(format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", uuid))?.error_for_status()?.json()?)
		} else {
			Ok(response.json()?)
		}
	}
	fn enter_play(mut self, session: super::login::SessionResponse) -> anyhow::Result<()> {
		trace!("Sending login success packet");
		let packet = crate::packets::login::send::Packet::LoginSuccess {
			uuid: UuidWrapper(session.uuid),
			username: PrefixedString(session.username),
		};
		self.send_packet(&packet)?;
		self.handle_play()
	}

	pub(super) fn handle_login(mut self) -> anyhow::Result<()> {
		trace!("Entering login state");
		let username = self.receive_login_start()?;
		trace!("Connection username: {}", username);
		// TODO offline mode
		let verify_token = self.request_encryption()?;
		let shared_secret = self.receive_shared_secret(verify_token)?;
		let session_response = self.get_session(username, &shared_secret)?;
		let cipher = Self::make_cipher(&shared_secret)?;
		self.socket = Box::new(CipherWrapper::new(self.socket, cipher));
		self.enter_play(session_response)
	}
}
