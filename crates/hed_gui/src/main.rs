// https://doc.rust-lang.org/reference/runtime.html?highlight=windows_subsystem#the-windows_subsystem-attribute
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod util;

use anyhow::Result;
use eframe::{egui, egui_wgpu};

use app::App;

fn main() {
	if let Err(err) = run() {
		eprintln!("{}", err);
	}
}

fn run() -> Result<()> {
	eframe::run_native(
		"Hed",
		create_native_options(),
		Box::new(|_| {
			let app = App::new()?;
			Ok(Box::new(app))
		}),
	)?;

	Ok(())
}

fn create_native_options() -> eframe::NativeOptions {
	eframe::NativeOptions {
		viewport: create_viewport_builder(),
		wgpu_options: create_wgpu_options(),
		centered: true,
		..Default::default()
	}
}

fn create_viewport_builder() -> egui::ViewportBuilder {
	const MIN_SIZE: [f32; 2] = [800.0, 600.0];

	egui::ViewportBuilder::default()
		.with_min_inner_size(MIN_SIZE)
		.with_inner_size(MIN_SIZE)
}

fn create_wgpu_options() -> egui_wgpu::WgpuConfiguration {
	egui_wgpu::WgpuConfiguration {
		power_preference: egui_wgpu::wgpu::PowerPreference::LowPower,
		..Default::default()
	}
}
