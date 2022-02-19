use super::Client;
use crate::packets::helpers::wrappers::json::Json;
use crate::packets::status::receive::Packet as Receive;
use crate::packets::status::send::{self, Packet as Send};
use log::trace;

impl Client {
	pub(super) fn handle_status(mut self) -> anyhow::Result<()> {
		trace!("Entering status state");
		loop {
			let packet = self.receive_packet()?;
			let response = match packet {
				// client is free to close the connection at any time
				None => {
					return Ok(());
				}
				Some(Receive::Ping(echo)) => Send::Pong(echo),
				Some(Receive::RequestStatus) => Send::ReplyStatus(Json(send::StatusReply {
					version: send::StatusVersion {
						name: super::SERVER_VERSION.to_string(),
						protocol: super::PROTOCOL_VERSION,
					},
					players: send::StatusPlayers { max: 420, online: 69, sample: None },
					description: send::StatusDescription { text: &self.config.listing.motd },
					favicon: self.config.listing.icon.as_deref(),
				})),
			};
			self.send_packet(&response)?;
		}
	}
}
