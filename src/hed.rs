use std::path::PathBuf;

use anyhow::Result;
use eframe::egui;

use crate::{
	core::{Invoke, Profile, Response, TaskHandler},
	ui::{editor, editor_header, header, left_side},
	util::get_sys_hosts_path,
};

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
	pub new_profile_esc: bool,
}

impl Hed {
	pub fn new() -> Result<Self> {
		let task_handler = TaskHandler::new();
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
			new_profile_esc: false,
		})
	}

	pub fn init(&mut self) {
		self.invoke_parse_hosts();
	}

	fn handle_task_response(&mut self) {
		if let Ok(resp) = self.task_handler.rx.try_recv() {
			match resp {
				Response::Parse(hosts_info) => {
					if let Some(profile) = self
						.profiles
						.iter_mut()
						.find(|p| p.id == self.enabled_profile_id)
					{
						profile.hosts_info = hosts_info;
					}
					self.profiles_loading = false;
				}
				Response::ParseFail(err) => {
					self.profiles_loading = false;
					eprintln!("{}", err);
				}
			}
		}
	}

	fn invoke_parse_hosts(&mut self) {
		self.task_handler
			.invoke(Invoke::Parse(self.sys_hosts_path.clone()));
		self.profiles_loading = true;
	}

	pub fn open_new_profile_window(&mut self) {
		self.new_profile_open = true;
		self.new_profile_name.clear();
	}

	pub fn create_profile(&mut self) {
		self.profiles.push(Profile::new(&self.new_profile_name));
		self.reset_new_profile_state();
	}

	pub fn reset_new_profile_state(&mut self) {
		self.new_profile_name.clear();
		self.new_profile_open = false;
		self.new_pofile_ok = false;
		self.new_profile_esc = false;
	}
}

impl eframe::App for Hed {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		header(ctx, self);
		left_side(ctx, self);
		editor_header(ctx, self);
		editor(ctx, self);
		self.handle_task_response();
	}
}
