use eframe::egui;
use hed_core::Hed;

use crate::util::get_sys_hosts_path;

pub struct App {
	hed: Hed,
}

impl App {
	pub fn new() -> Self {
		let hosts_path = get_sys_hosts_path().unwrap();
		let hed = Hed::new(hosts_path);

		Self { hed }
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("My egui Application");
			ui.label(format!("hosts path: {}", self.hed.hosts_path.display()));
		});
	}
}
