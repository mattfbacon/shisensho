use super::Client;
use crate::packets::play::{receive::Packet as Receive, send::Packet as Send};
use log::debug;

impl Client {
	pub(super) fn handle_play(mut self) -> anyhow::Result<()> {
		debug!("Entering play state");
		Ok(())
	}
}
