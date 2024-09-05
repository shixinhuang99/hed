use std::path::PathBuf;

use crate::{
	channel::{hed_channel, ChannelEvent, HedReceiver, HedSender, InvokeEvent},
	hosts_info,
};

pub struct Hed {
	tx: HedSender,
	pub rx: HedReceiver,
	pub hosts_path: PathBuf,
}

impl Hed {
	pub fn new(hosts_path: PathBuf) -> Self {
		let (tx, rx) = hed_channel();

		Self { tx, rx, hosts_path }
	}

	pub fn invoke(&self, event: InvokeEvent) {
		match event {
			InvokeEvent::Parse => {
				tokio::spawn(parse_hosts_task(
					self.tx.clone(),
					self.hosts_path.clone(),
				));
			}
		}
	}
}

async fn parse_hosts_task(tx: HedSender, hosts_path: PathBuf) {
	let hosts_info = hosts_info::parse_hosts(hosts_path).await.unwrap();
	let _ = tx.send(ChannelEvent::Parse(hosts_info)).await;
}
