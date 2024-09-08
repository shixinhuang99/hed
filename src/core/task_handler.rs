use std::path::PathBuf;

use anyhow::Result;
use tokio::sync::mpsc::channel;

use super::{
	channel::{Invoke, Receiver, Response, Sender},
	HostsInfo,
};

pub struct TaskHandler {
	tx: Sender,
	pub rx: Receiver,
}

impl Default for TaskHandler {
	fn default() -> Self {
		let (tx, rx) = channel(100);

		Self { tx, rx }
	}
}

impl TaskHandler {
	pub fn invoke(&self, event: Invoke) {
		match event {
			Invoke::Parse(hosts_path) => {
				tokio::spawn(parse_hosts_task(self.tx.clone(), hosts_path));
			}
		}
	}
}

async fn parse_hosts_task(tx: Sender, hosts_path: PathBuf) -> Result<()> {
	match HostsInfo::parse_from_file(hosts_path).await {
		Ok(hosts_info) => {
			tx.send(Response::Parse(hosts_info)).await?;
		}
		Err(err) => {
			tx.send(Response::ParseFail(err.to_string())).await?;
		}
	}

	Ok(())
}
