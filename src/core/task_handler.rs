use std::{path::PathBuf, sync::mpsc, thread};

use anyhow::Result;

use super::HostsInfo;

type Sender = mpsc::Sender<Response>;
type Receiver = mpsc::Receiver<Response>;

pub enum Invoke {
	ParseHosts(PathBuf),
}

pub enum Response {
	ParseHostsOk(HostsInfo),
	ParseHostsFail(String),
}

pub struct TaskHandler {
	tx: Sender,
	rx: Receiver,
}

impl Default for TaskHandler {
	fn default() -> Self {
		let (tx, rx) = mpsc::channel();

		Self { tx, rx }
	}
}

impl TaskHandler {
	pub fn invoke(&self, invoke: Invoke) {
		use Invoke::*;

		match invoke {
			ParseHosts(hosts_path) => {
				parse_hosts_task(self.tx.clone(), hosts_path);
			}
		}
	}

	pub fn recv(&self) -> Option<Response> {
		self.rx.try_recv().ok()
	}
}

fn parse_hosts_task(tx: Sender, hosts_path: PathBuf) {
	thread::spawn(move || -> Result<()> {
		match HostsInfo::parse_from_file(hosts_path) {
			Ok(hosts_info) => {
				tx.send(Response::ParseHostsOk(hosts_info))?;
			}
			Err(err) => {
				tx.send(Response::ParseHostsFail(err.to_string()))?;
			}
		}
		Ok(())
	});
}
