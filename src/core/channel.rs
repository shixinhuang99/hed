use std::path::PathBuf;

use tokio::sync::mpsc;

use super::HostsInfo;

pub type Sender = mpsc::Sender<Response>;
pub type Receiver = mpsc::Receiver<Response>;

pub enum Invoke {
	Parse(PathBuf),
}

pub enum Response {
	Parse(HostsInfo),
	ParseFail(String),
}
