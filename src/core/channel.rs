use std::sync::mpsc;

use super::HostsInfo;

pub enum Event {
	ParseHostsOk(HostsInfo),
	OsErr(String),
	EditItemIp(usize, String),
	ToggleHostEnable(usize, usize),
	DeleteItem(usize),
	DeleteHost(usize, usize),
	OpenAddHostsWindow(usize),
	OpenEditHostWindow(usize, usize),
	SaveHostsOk,
	ToggleAllHostEnable(usize, bool),
}

pub struct Channel {
	pub tx: mpsc::Sender<Event>,
	rx: mpsc::Receiver<Event>,
}

impl Default for Channel {
	fn default() -> Self {
		let (tx, rx) = mpsc::channel();

		Self { tx, rx }
	}
}

impl Channel {
	pub fn send(&self, event: Event) {
		self.tx.send(event).unwrap();
	}

	pub fn recv(&self) -> Option<Event> {
		self.rx.try_recv().ok()
	}
}
