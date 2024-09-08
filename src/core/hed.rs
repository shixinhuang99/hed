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
	pub new_profile_open: bool,
	pub new_profile_name: String,
	pub new_pofile_ok: bool,
	pub new_profile_err: bool,
	pub mark_deleted_profile_id: Option<usize>,
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
			new_profile_open: false,
			new_profile_name: String::new(),
			new_pofile_ok: false,
			new_profile_err: false,
			mark_deleted_profile_id: None,
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
		if self.new_profile_name.trim().is_empty() {
			self.new_pofile_ok = false;
			return;
		}
		if self
			.profiles
			.iter()
			.any(|p| p.name == self.new_profile_name)
		{
			self.new_profile_err = true;
			self.new_pofile_ok = false;
			return;
		}
		self.profiles.push(Profile::new(&self.new_profile_name));
		self.close_new_profile_window();
	}

	pub fn close_new_profile_window(&mut self) {
		self.new_profile_name.clear();
		self.new_profile_open = false;
		self.new_pofile_ok = false;
		self.new_profile_err = false;
	}

	pub fn get_display_porfiles(&self) -> Vec<&Profile> {
		self.profiles
			.iter()
			.filter(|p| p.name.contains(self.search_profile.trim()))
			.collect()
	}

	pub fn check_profile_deleted(&mut self) {
		if let Some(id) = self.mark_deleted_profile_id {
			self.profiles.retain(|p| p.id != id);
			self.mark_deleted_profile_id = None;
		}
	}
}
