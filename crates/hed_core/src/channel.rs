use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::hosts_info::HostsInfo;

pub(crate) type HedSender = Sender<ChannelEvent>;
pub(crate) type HedReceiver = Receiver<ChannelEvent>;

pub enum InvokeEvent {
	Parse,
}

pub enum ChannelEvent {
	Parse(HostsInfo),
}

pub(crate) fn hed_channel() -> (HedSender, HedReceiver) {
	channel(100)
}
