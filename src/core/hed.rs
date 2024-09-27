use std::{path::PathBuf, thread};

use anyhow::Result;

use super::{
	channel::{Channel, Event},
	item_form::ItemForm,
	HostsInfo,
};
use crate::util::{get_sys_hosts_path, StringExt};

#[derive(Default)]
pub struct Hed {
	channel: Channel,
	#[cfg(feature = "_dev")]
	pub dev_window_open: bool,
	pub sys_hosts_path: PathBuf,
	pub sys_hosts_loading: bool,
	pub os_err: String,
	pub hosts_info: HostsInfo,
	pub hosts_info_draft: HostsInfo,
	pub view_kind: ViewKind,
	pub view_all: bool,
	pub search_ip_hosts: String,
	pub item_form: ItemForm,
	pub opened_window: Option<OpenedWindow>,
	selected_item_id: Option<usize>,
	selected_host_id: Option<usize>,
}

#[derive(PartialEq, Eq, Default)]
pub enum ViewKind {
	#[default]
	Options,
	Text,
}

#[derive(PartialEq, Eq)]
pub enum OpenedWindow {
	NewItem,
	AddHosts,
	EditHost,
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
				OsErr(err) => {
					self.handle_os_err(err);
				}
				EditItemIp(item_id, ip) => {
					self.edit_item_ip(item_id, ip);
				}
				ToggleHostEnable(item_id, host_id) => {
					self.toggle_host_enable(item_id, host_id);
				}
				DeleteItem(item_id) => {
					self.delete_item(item_id);
				}
				DeleteHost(item_id, host_id) => {
					self.delete_host(item_id, host_id);
				}
				OpenAddHostsWindow(item_id) => {
					self.open_add_hosts_window(item_id);
				}
				OpenEditHostWindow(item_id, host_id) => {
					self.open_edit_host_window(item_id, host_id);
				}
				SaveHostsOk => {
					self.parse_sys_hosts();
				}
			}
		}
	}

	pub fn send_event(&self, event: Event) {
		self.channel.send(event);
	}

	fn parse_sys_hosts(&mut self) {
		if self.sys_hosts_path.to_string_lossy().is_empty() {
			let Ok(sys_hosts_path) = get_sys_hosts_path() else {
				self.os_err =
					"Failed to get the path of system hosts file".to_string();
				return;
			};
			self.sys_hosts_path = sys_hosts_path.clone();
		}
		self.sys_hosts_loading = true;
		let tx = self.channel.tx.clone();
		let hosts_path = self.sys_hosts_path.clone();
		thread::spawn(move || -> Result<()> {
			match HostsInfo::parse_from_file(hosts_path) {
				Ok(hosts_info) => {
					tx.send(Event::ParseHostsOk(hosts_info))?;
				}
				Err(err) => {
					tx.send(Event::OsErr(err.to_string()))?;
				}
			}
			Ok(())
		});
	}

	fn handle_parse_hosts_ok(&mut self, hosts_info: HostsInfo) {
		self.hosts_info_draft.clone_from(&hosts_info);
		self.hosts_info.clone_from(&hosts_info);
		self.sys_hosts_loading = false;
	}

	fn handle_os_err(&mut self, err: String) {
		self.os_err = err;
		self.sys_hosts_loading = false;
	}

	pub fn close_item_form_window(&mut self) {
		self.opened_window = None;
		self.item_form.reset();
	}

	pub fn new_item(&mut self) {
		if !self.item_form.validate() {
			return;
		}
		self.hosts_info_draft.add_item(&self.item_form);
		self.update_content();
		self.close_item_form_window();
	}

	pub fn save_hosts(&mut self) {
		self.sys_hosts_loading = true;
		let tx = self.channel.tx.clone();
		let hosts_path = self.sys_hosts_path.clone();
		let hosts_info = self.hosts_info_draft.clone();
		thread::spawn(move || -> Result<()> {
			if let Err(err) = hosts_info.save_to_file(hosts_path) {
				tx.send(Event::OsErr(err.to_string()))?;
			} else {
				tx.send(Event::SaveHostsOk)?;
			}
			Ok(())
		});
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

	pub fn set_opened_window(&mut self, window: OpenedWindow) {
		self.opened_window = Some(window);
	}

	pub fn is_window_open(&self, window: OpenedWindow) -> bool {
		if let Some(win) = &self.opened_window {
			return *win == window;
		}
		false
	}

	fn edit_item_ip(&mut self, item_id: usize, ip: String) {
		if let Some(item) = self.hosts_info_draft.get_item_mut(item_id) {
			if item.validate_ip(&ip) {
				item.ip = ip;
			}
		}
		self.update_content();
	}

	fn toggle_host_enable(&mut self, item_id: usize, host_id: usize) {
		if let Some(item) = self.hosts_info_draft.get_item_mut(item_id) {
			if let Some(host) = item.get_host_mut(host_id) {
				host.enabled = !host.enabled;
			}
		}
		self.update_content();
	}

	fn delete_item(&mut self, item_id: usize) {
		self.hosts_info_draft.remove_item(item_id);
		self.update_content();
	}

	fn delete_host(&mut self, item_id: usize, host_id: usize) {
		let Some(item) = self.hosts_info_draft.get_item_mut(item_id) else {
			return;
		};
		item.remove_host(host_id);
		if item.hosts.is_empty() {
			self.hosts_info_draft.remove_item(item_id);
		}
		self.update_content();
	}

	fn open_add_hosts_window(&mut self, item_id: usize) {
		self.selected_item_id = Some(item_id);
		self.set_opened_window(OpenedWindow::AddHosts);
	}

	pub fn close_add_hosts_window(&mut self) {
		self.selected_item_id = None;
		self.close_item_form_window();
	}

	pub fn add_hosts(&mut self) {
		if !self.item_form.validate_hosts() {
			return;
		}
		let Some(item_id) = self.selected_item_id else {
			return;
		};
		let Some(item) = self.hosts_info_draft.get_item_mut(item_id) else {
			return;
		};
		item.add_hosts(self.item_form.hosts.to_split_whitespace_vec(), true);
		self.close_add_hosts_window();
		self.update_content();
	}

	fn open_edit_host_window(&mut self, item_id: usize, host_id: usize) {
		self.selected_item_id = Some(item_id);
		self.selected_host_id = Some(host_id);
		if let Some(item) = self.hosts_info_draft.get_item_mut(item_id) {
			if let Some(host) = item.get_host_mut(host_id) {
				self.item_form.hosts = host.name.clone();
			};
		};
		self.set_opened_window(OpenedWindow::EditHost);
	}

	pub fn close_edit_host_window(&mut self) {
		self.selected_item_id = None;
		self.selected_host_id = None;
		self.close_item_form_window();
	}

	pub fn edit_host(&mut self) {
		if !self.item_form.validate_hosts() {
			return;
		}
		let (Some(item_id), Some(host_id)) =
			(self.selected_item_id, self.selected_host_id)
		else {
			return;
		};
		let Some(item) = self.hosts_info_draft.get_item_mut(item_id) else {
			return;
		};
		item.rename_host(host_id, self.item_form.hosts.clone());
		self.close_edit_host_window();
		self.update_content();
	}
}
