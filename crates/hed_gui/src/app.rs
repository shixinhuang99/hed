use anyhow::Result;
use eframe::egui;
use hed_core::{
	channel::{ChannelEvent, InvokeEvent},
	Hed, HostsInfo,
};

use crate::util::get_sys_hosts_path;

pub struct App {
	hed: Hed,
	hosts_info: HostsInfo,
	hosts_info_loading: bool,
}

impl App {
	pub fn new() -> Result<Self> {
		let hosts_path = get_sys_hosts_path()?;
		let hed = Hed::new(hosts_path);
		let hosts_info = HostsInfo::default();

		Ok(Self {
			hed,
			hosts_info,
			hosts_info_loading: false,
		})
	}

	pub fn init(&mut self) {
		self.invoke_parse_hosts();
	}

	fn handle_channel_event(&mut self) {
		if let Ok(event) = self.hed.rx.try_recv() {
			match event {
				ChannelEvent::Parse(hosts_info) => {
					self.hosts_info = hosts_info;
					self.hosts_info_loading = false;
				}
			}
		}
	}

	fn invoke_parse_hosts(&mut self) {
		self.hed.invoke(InvokeEvent::Parse);
		self.hosts_info_loading = true;
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Hed");
			ui.label(format!("hosts path: {}", self.hed.hosts_path.display()));
			if ui.button("parse").clicked() {
				self.invoke_parse_hosts();
			};
			if self.hosts_info_loading {
				ui.spinner();
			} else {
				ui.text_edit_multiline(&mut self.hosts_info.content);
			}
		});

		self.handle_channel_event();
	}
}
