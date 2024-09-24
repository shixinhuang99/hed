use std::{path::PathBuf, thread};

use anyhow::Result;

use super::{
	channel::{Channel, Event},
	item_form::ItemForm,
	view_kind::ViewKind,
	HostsInfo,
};
use crate::util::get_sys_hosts_path;

#[derive(Default)]
pub struct Hed {
	channel: Channel,
	#[cfg(feature = "_dev")]
	pub dev_window_open: bool,
	pub sys_hosts_path: PathBuf,
	pub sys_hosts_loading: bool,
	pub parse_sys_hosts_err: String,
	pub hosts_info: HostsInfo,
	pub hosts_info_draft: HostsInfo,
	pub view_kind: ViewKind,
	pub view_all: bool,
	pub search_ip_hosts: String,
	pub new_item_window_open: bool,
	pub item_form: ItemForm,
}

impl Hed {
	pub fn init(&mut self) {
		self.parse_sys_hosts();
	}

	pub fn handle_event(&mut self) {
		use Event::*;

		if let Some(event) = self.channel.recv() {
			match event {
				ParseHostsOk(hosts_info) => {
					self.handle_parse_hosts_ok(hosts_info);
				}
				ParseHostsFail(err) => {
					self.handle_parse_hosts_fail(err);
				}
				EditItemIp(item_id, ip) => {
					self.handle_edit_item_ip(item_id, ip);
					self.update_content();
				}
				ToggleHostEnable(item_id, host_id) => {
					self.handle_toggle_host_enable(item_id, host_id);
					self.update_content();
				}
			}
		}
	}

	pub fn send_event(&self, event: Event) {
		self.channel.send(event);
	}

	fn parse_sys_hosts(&mut self) {
		let Ok(sys_hosts_path) = get_sys_hosts_path() else {
			self.parse_sys_hosts_err =
				"Failed to get the path of system hosts file".to_string();
			return;
		};
		self.sys_hosts_path = sys_hosts_path.clone();
		let tx = self.channel.tx.clone();
		thread::spawn(move || -> Result<()> {
			match HostsInfo::parse_from_file(sys_hosts_path) {
				Ok(hosts_info) => {
					tx.send(Event::ParseHostsOk(hosts_info))?;
				}
				Err(err) => {
					tx.send(Event::ParseHostsFail(err.to_string()))?;
				}
			}
			Ok(())
		});
		self.sys_hosts_loading = true;
	}

	fn handle_parse_hosts_ok(&mut self, hosts_info: HostsInfo) {
		self.hosts_info_draft.clone_from(&hosts_info);
		self.hosts_info.clone_from(&hosts_info);
		self.sys_hosts_loading = false;
	}

	fn handle_parse_hosts_fail(&mut self, err: String) {
		self.parse_sys_hosts_err = err;
		self.sys_hosts_loading = false;
	}

	pub fn close_new_item_window(&mut self) {
		self.new_item_window_open = false;
		self.item_form.reset();
	}

	pub fn new_item(&mut self) {
		if !self.item_form.validate() {
			return;
		}
		self.hosts_info_draft.add_item(&self.item_form);
		self.close_new_item_window();
	}

	pub fn save_hosts(&mut self) {
		self.hosts_info.clone_from(&self.hosts_info_draft);
	}

	pub fn reset_hosts(&mut self) {
		self.hosts_info_draft.clone_from(&self.hosts_info);
	}

	pub fn is_hosts_changed(&self) -> bool {
		self.hosts_info_draft.content != self.hosts_info.content
	}

	pub fn update_list(&mut self) {
		self.hosts_info_draft.update_list();
	}

	pub fn update_content(&mut self) {
		self.hosts_info_draft.update_content();
	}

	fn handle_edit_item_ip(&mut self, item_id: usize, ip: String) {
		if let Some(item) = self
			.hosts_info_draft
			.list
			.iter_mut()
			.find(|item| item.id == item_id)
		{
			if item.validate_ip(&ip) {
				item.ip = ip;
			}
		}
	}

	fn handle_toggle_host_enable(&mut self, item_id: usize, host_id: usize) {
		if let Some(item) = self
			.hosts_info_draft
			.list
			.iter_mut()
			.find(|item| item.id == item_id)
		{
			if let Some(host) =
				item.hosts.iter_mut().find(|host| host.id == host_id)
			{
				host.enabled = !host.enabled;
			}
		}
	}
}
