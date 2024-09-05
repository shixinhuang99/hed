use std::path::PathBuf;

use anyhow::Result;
use eframe::egui;

use crate::{
	core::{HostsInfo, Invoke, Profile, Response, TaskHandler},
	util::get_sys_hosts_path,
};

pub struct Hed {
	task_handler: TaskHandler,
	hosts_path: PathBuf,
	system_profile: Profile,
	hosts_info: HostsInfo,
	hosts_info_loading: bool,
}

impl Hed {
	pub fn new() -> Result<Self> {
		let task_handler = TaskHandler::new();
		let hosts_path = get_sys_hosts_path()?;
		let hosts_info = HostsInfo::default();

		Ok(Self {
			task_handler,
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
				Response::ParseFail(_) => {
					self.hosts_info_loading = false;
					todo!();
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
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Hed");
			ui.label(format!("hosts path: {}", self.hosts_path.display()));
			if ui.button("parse").clicked() {
				self.invoke_parse_hosts();
			};
			if self.hosts_info_loading {
				ui.spinner();
			} else {
				ui.text_edit_multiline(&mut self.hosts_info.content);
			}
		});

		self.handle_task_response();
	}
}
