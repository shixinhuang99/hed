use std::path::PathBuf;

use anyhow::Result;

use super::{
	task_handler::{Invoke, Response, TaskHandler},
	Profile,
};
use crate::util::get_sys_hosts_path;

pub struct Hed {
	task_handler: TaskHandler,
	#[cfg(feature = "_dev")]
	pub dev_window_open: bool,
	pub sys_hosts_path: PathBuf,
	pub profiles: Vec<Profile>,
	pub profiles_loading: bool,
	pub enabled_profile_id: usize,
	pub selected_profile_id: usize,
	pub search_profile: String,
	pub new_profile_window_open: bool,
	pub new_profile_name: String,
	pub new_profile_err: String,
}

impl Hed {
	pub fn new() -> Result<Self> {
		let task_handler = TaskHandler::default();
		let sys_hosts_path = get_sys_hosts_path()?;
		let sys_profile = Profile::new_system();

		Ok(Self {
			task_handler,
			#[cfg(feature = "_dev")]
			dev_window_open: false,
			sys_hosts_path,
			enabled_profile_id: sys_profile.id,
			selected_profile_id: sys_profile.id,
			profiles: vec![sys_profile],
			profiles_loading: false,
			search_profile: String::new(),
			new_profile_window_open: false,
			new_profile_name: String::new(),
			new_profile_err: String::new(),
		})
	}

	pub fn init(&mut self) {
		self.parse_sys_hosts();
	}

	pub fn handle_task_response(&mut self) {
		use Response::*;

		if let Some(resp) = self.task_handler.recv() {
			match resp {
				ParseHostsOk(hosts_info) => {
					if let Some(profile) = self
						.profiles
						.iter_mut()
						.find(|p| p.id == self.enabled_profile_id)
					{
						profile.content_draft = hosts_info.content.clone();
						profile.hosts_info = hosts_info;
					}
					self.profiles_loading = false;
				}
				ParseHostsFail(err) => {
					self.profiles_loading = false;
					eprintln!("{}", err);
				}
			}
		}
	}

	fn parse_sys_hosts(&mut self) {
		let path = self.sys_hosts_path.clone();
		self.task_handler.invoke(Invoke::ParseHosts(path));
		self.profiles_loading = true;
	}

	pub fn create_profile(&mut self) {
		let new_name = self.new_profile_name.trim();
		if new_name.is_empty() {
			return;
		}
		if self.profiles.iter().any(|p| p.name == new_name) {
			self.new_profile_err = format!("`{}` already exists", new_name);
			return;
		}
		self.profiles.push(Profile::new(new_name));
		self.close_new_profile_window();
	}

	pub fn close_new_profile_window(&mut self) {
		self.new_profile_name.clear();
		self.new_profile_err.clear();
		self.new_profile_window_open = false;
	}

	pub fn remove_profile(&mut self, id: usize) {
		self.profiles.retain(|p| p.id != id);
		if self.selected_profile_id == id {
			self.selected_profile_id = self.profiles[0].id;
		}
	}

	pub fn selected_profile_mut(&mut self) -> Option<&mut Profile> {
		self.profiles
			.iter_mut()
			.find(|p| p.id == self.selected_profile_id)
	}
}
