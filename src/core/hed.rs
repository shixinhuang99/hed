use std::{path::PathBuf, thread};

use anyhow::Result;

use super::{
	channel::{Channel, Event},
	item_form::ItemForm,
	profile::Profile,
	profile_form::ProfileForm,
	view_kind::ViewKind,
	HostsInfo,
};
use crate::util::get_sys_hosts_path;

pub struct Hed {
	channel: Channel,
	#[cfg(feature = "_dev")]
	pub dev_window_open: bool,
	pub sys_hosts_path: PathBuf,
	pub profiles: Vec<Profile>,
	pub profiles_loading: bool,
	pub enabled_profile_id: usize,
	pub selected_profile_id: usize,
	pub search_profile: String,
	pub new_profile_window_open: bool,
	pub new_profile_form: ProfileForm,
	pub edit_profile_id: Option<usize>,
	pub edit_profile_form: ProfileForm,
	pub view_kind: ViewKind,
	pub search_ip_hosts: String,
	pub new_item_window_open: bool,
	pub item_form: ItemForm,
}

impl Hed {
	pub fn new() -> Result<Self> {
		let channel = Channel::default();
		let sys_hosts_path = get_sys_hosts_path()?;
		let sys_profile = Profile::new_system();

		Ok(Self {
			channel,
			#[cfg(feature = "_dev")]
			dev_window_open: false,
			sys_hosts_path,
			enabled_profile_id: sys_profile.id,
			selected_profile_id: sys_profile.id,
			profiles: vec![sys_profile],
			profiles_loading: false,
			search_profile: String::new(),
			new_profile_window_open: false,
			new_profile_form: ProfileForm::default(),
			edit_profile_id: None,
			edit_profile_form: ProfileForm::default(),
			view_kind: ViewKind::default(),
			search_ip_hosts: String::new(),
			new_item_window_open: false,
			item_form: ItemForm::default(),
		})
	}

	pub fn init(&mut self) {
		self.parse_sys_hosts();
	}

	pub fn handle_event(&mut self) {
		use Event::*;

		if let Some(resp) = self.channel.recv() {
			match resp {
				ParseHostsOk(hosts_info) => {
					self.handle_parse_hosts_ok(hosts_info);
				}
				ParseHostsFail(err) => {
					self.profiles_loading = false;
					eprintln!("{}", err);
				}
				EnableProfile(id) => {
					self.enabled_profile_id = id;
					self.selected_profile_id = id;
				}
				SelectProfile(id) => {
					self.selected_profile_id = id;
				}
				RemoveProfile(id) => {
					self.remove_profile(id);
				}
				EditProfile(id) => {
					self.edit_profile_id = Some(id);
					if let Some(profile) =
						self.profiles.iter().find(|p| p.id == id)
					{
						self.edit_profile_form.name = profile.name.clone();
					}
				}
				ToggleViewKind => {
					self.view_kind.toogle();
				}
				SavePorfile => {
					if let Some(profile) = self.get_selected_profile_mut() {
						profile.save_content();
					}
				}
				ResetProfile => {
					if let Some(profile) = self.get_selected_profile_mut() {
						profile.reset_content();
					}
				}
				ToggleHostEnable(ih_id, host_id) => {
					let Some(profile) = self.get_selected_profile_mut() else {
						return;
					};
					let Some(ip_hosts) = profile
						.hosts_info_draft
						.list
						.iter_mut()
						.find(|ih| ih.id == ih_id)
					else {
						return;
					};
					let Some(host) =
						ip_hosts.hosts.iter_mut().find(|h| h.id == host_id)
					else {
						return;
					};
					host.enabled = !host.enabled;
				}
			}
		}
	}

	pub fn send_event(&self, event: Event) {
		self.channel.send(event);
	}

	fn parse_sys_hosts(&mut self) {
		let hosts_path = self.sys_hosts_path.clone();
		let tx = self.channel.tx.clone();
		thread::spawn(move || -> Result<()> {
			match HostsInfo::parse_from_file(hosts_path) {
				Ok(hosts_info) => {
					tx.send(Event::ParseHostsOk(hosts_info))?;
				}
				Err(err) => {
					tx.send(Event::ParseHostsFail(err.to_string()))?;
				}
			}
			Ok(())
		});
		self.profiles_loading = true;
	}

	fn handle_parse_hosts_ok(&mut self, hosts_info: HostsInfo) {
		if let Some(profile) = self
			.profiles
			.iter_mut()
			.find(|p| p.id == self.enabled_profile_id)
		{
			profile.hosts_info_draft = hosts_info.clone();
			profile.hosts_info = hosts_info;
		}
		self.profiles_loading = false;
	}

	pub fn create_profile(&mut self) {
		if self
			.new_profile_form
			.validate(|name| self.profiles.iter().any(|p| p.name == name))
		{
			self.profiles
				.push(Profile::new(&self.new_profile_form.name));
			self.close_new_profile_window();
		}
	}

	pub fn close_new_profile_window(&mut self) {
		self.new_profile_window_open = false;
		self.new_profile_form.reset();
	}

	fn remove_profile(&mut self, id: usize) {
		self.profiles.retain(|p| p.id != id);
		if self.selected_profile_id == id {
			self.selected_profile_id = self.profiles[0].id;
		}
	}

	pub fn get_display_profiles(&self) -> Vec<&Profile> {
		self.profiles
			.iter()
			.filter(|p| p.name.contains(self.search_profile.trim()))
			.collect()
	}

	pub fn edit_profile(&mut self) {
		let Some(id) = self.edit_profile_id else {
			return;
		};
		if self.edit_profile_form.validate(|name| {
			self.profiles.iter().any(|p| p.id != id && p.name == name)
		}) {
			if let Some(profile) = self.profiles.iter_mut().find(|p| p.id == id)
			{
				profile.name = self.edit_profile_form.name.clone();
				self.close_edit_profile_window();
			}
		}
	}

	pub fn close_edit_profile_window(&mut self) {
		self.edit_profile_id = None;
		self.edit_profile_form.reset();
	}

	pub fn get_selected_profile(&self) -> Option<&Profile> {
		self.profiles
			.iter()
			.find(|p| p.id == self.selected_profile_id)
	}

	pub fn get_selected_profile_mut(&mut self) -> Option<&mut Profile> {
		self.profiles
			.iter_mut()
			.find(|p| p.id == self.selected_profile_id)
	}

	pub fn close_new_item_window(&mut self) {
		self.new_item_window_open = false;
		self.item_form.reset();
	}

	pub fn new_item(&mut self) {
		if !self.item_form.validate() {
			return;
		}
		let Some(profile) = self
			.profiles
			.iter_mut()
			.find(|p| p.id == self.selected_profile_id)
		else {
			return;
		};
		profile.hosts_info_draft.add_item(&self.item_form);
		self.close_new_item_window();
	}
}
