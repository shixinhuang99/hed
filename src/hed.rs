use std::path::PathBuf;

use anyhow::Result;
use eframe::egui;

use crate::{
	core::{HostsInfo, Invoke, Profile, Response, TaskHandler},
	ui::{editor, editor_header, header, left_side},
	util::get_sys_hosts_path,
};

pub struct Hed {
	task_handler: TaskHandler,
	#[cfg(feature = "_dev")]
	pub dev_window_open: bool,
	pub hosts_path: PathBuf,
	pub system_profile: Profile,
	pub hosts_info: HostsInfo,
	pub hosts_info_loading: bool,
}

impl Hed {
	pub fn new() -> Result<Self> {
		let task_handler = TaskHandler::new();
		let hosts_path = get_sys_hosts_path()?;
		let hosts_info = HostsInfo::default();

		Ok(Self {
			task_handler,
			#[cfg(feature = "_dev")]
			dev_window_open: false,
			hosts_path,
			system_profile: Profile::new_system(),
			hosts_info,
			hosts_info_loading: false,
		})
	}

	pub fn init(&mut self) {
		self.invoke_parse_hosts();
	}

	fn handle_task_response(&mut self) {
		if let Ok(resp) = self.task_handler.rx.try_recv() {
			match resp {
				Response::Parse(hosts_info) => {
					self.hosts_info = hosts_info;
					self.hosts_info_loading = false;
				}
				Response::ParseFail(err) => {
					self.hosts_info_loading = false;
					eprintln!("{}", err);
				}
			}
		}
	}

	fn invoke_parse_hosts(&mut self) {
		self.task_handler
			.invoke(Invoke::Parse(self.hosts_path.clone()));
		self.hosts_info_loading = true;
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
