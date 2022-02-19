use crate::config::Config;
use crate::packets::handshake::receive::Packet as HandshakeReceive;
use crate::packets::helpers::varint::VarInt;
use crate::packets::ProtocolState;
use encde::{Decode, DecodeSized, Encode};
use log::{debug, info, trace};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::{SocketAddr, TcpStream};

mod login;
mod play;
mod status;

const PROTOCOL_VERSION: i32 = 757;
const SERVER_VERSION: &str = "1.18.1";

pub struct Server {
	config: &'static Config,
	global_state: &'static GlobalState,
}

impl Server {
	pub fn new(config: &'static Config) -> anyhow::Result<Self> {
		Ok(Self {
			config,
			global_state: Box::leak(Box::new(GlobalState::new()?)),
		})
	}
	pub fn listen(self) -> anyhow::Result<()> {
		let listener = TcpListener::bind((self.config.address, self.config.port))?;
		info!("Listening on {}:{}", self.config.address, self.config.port);
		loop {
			let (socket, client_address) = listener.accept()?;
			let client = Client::new(socket, client_address, self.config, self.global_state);
			std::thread::Builder::new().name(client_address.to_string()).spawn(move || {
				if let Err(err) = client.handle() {
					log::error!("{:#}", err);
				}
			})?;
		}
	}
}

struct GlobalState {
	rsa_key: openssl::rsa::Rsa<openssl::pkey::Private>,
	rsa_public_der: Vec<u8>,
}

impl GlobalState {
	fn new() -> anyhow::Result<Self> {
		debug!("Generating RSA key");
		let rsa_key = openssl::rsa::Rsa::generate(1024)?;
		debug!("Finished generating RSA key");
		let rsa_public_der = rsa_key.public_key_to_der()?;
		Ok(Self { rsa_public_der, rsa_key })
	}
}

trait ClientSocket: Read + Write + Send {}
impl<T: Read + Write + Send> ClientSocket for T {}

struct Client {
	socket: Box<dyn ClientSocket>,
	config: &'static Config,
	global_state: &'static GlobalState,
}

impl Client {
	fn receive_packet<P: DecodeSized>(&mut self) -> encde::Result<Option<P>> {
		let packet_len = match VarInt::decode(&mut self.socket) {
			Ok(VarInt(packet_len)) => packet_len.try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?,
			Err(encde::Error::Io(err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
			Err(err) => return Err(err),
		};
		let mut packet_data = vec![0u8; packet_len];
		self.socket.read_exact(&mut packet_data)?;
		encde::util::decode_from_entire_slice(&packet_data).map(Some)
	}
	fn send_packet<P: Encode>(&mut self, data: &P) -> encde::Result<()> {
		let packet_data = encde::util::encode_to_vec(data)?;
		VarInt(packet_data.len().try_into().map_err(|err| encde::Error::Custom(Box::new(err)))?).encode(&mut self.socket)?;
		self.socket.write_all(&packet_data)?;
		Ok(())
	}
}

impl Client {
	pub fn new(socket: TcpStream, address: SocketAddr, config: &'static crate::config::Config, global_state: &'static GlobalState) -> Self {
		trace!("New connection from {}", &address);
		Self {
			socket: Box::new(socket),
			config,
			global_state,
		}
	}
	pub fn handle(mut self) -> anyhow::Result<()> {
		let HandshakeReceive::Handshake(handshake) = self.receive_packet()?.ok_or_else(|| anyhow::anyhow!("Client closed connection").context("Handshake"))?;
		trace!("Client handshake: {:?}", handshake);
		match handshake.next_state {
			ProtocolState::Login => {
				anyhow::ensure!(
					handshake.protocol_version.0 == PROTOCOL_VERSION,
					"Client protocol version ({}) does not match ours ({}); disconnecting",
					handshake.protocol_version.0,
					PROTOCOL_VERSION
				);
				self.handle_login().map_err(|err| err.context("Login"))
			}
			ProtocolState::Status => self.handle_status().map_err(|err| err.context("Status")),
			unacceptable => {
				anyhow::bail!("Client requested an unacceptable next state ({:?}) in handshake", unacceptable);
			}
		}
	}
}
