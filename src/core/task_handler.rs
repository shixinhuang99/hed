use std::path::PathBuf;

use anyhow::Result;
use tokio::sync::mpsc::channel;

use crate::core::{
	channel::{Invoke, Receiver, Response, Sender},
	hosts_info::HostsInfo,
};

pub struct TaskHandler {
	tx: Sender,
	pub rx: Receiver,
}

impl TaskHandler {
	pub fn new() -> Self {
		let (tx, rx) = channel(100);

		Self { tx, rx }
	}

	pub fn invoke(&self, event: Invoke) {
		match event {
			Invoke::Parse(hosts_path) => {
				tokio::spawn(parse_hosts_task(self.tx.clone(), hosts_path));
			}
		}
	}
}

async fn parse_hosts_task(tx: Sender, hosts_path: PathBuf) -> Result<()> {
	if let Ok(hosts_info) = HostsInfo::parse_from_file(hosts_path).await {
		tx.send(Response::Parse(hosts_info)).await?;
	} else {
		tx.send(Response::ParseFail).await?;
	}

	Ok(())
}
