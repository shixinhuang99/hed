#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use eframe::egui_wgpu;

use hed::{consts::APP_NAME, Hed};

fn main() {
	if let Err(err) = run() {
		eprintln!("{}", err);
	}
}

fn run() -> Result<()> {
	eframe::run_native(
		APP_NAME,
		create_native_options(),
		Box::new(|_| {
			let mut hed = Hed::default();

			hed.init();

			Ok(Box::new(hed))
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
	let min_size = [1000.0, 700.0];
	let title: &str = "Hed";

	egui::ViewportBuilder::default()
		.with_min_inner_size(min_size)
		.with_inner_size(min_size)
		.with_title(title)
}

fn create_wgpu_options() -> egui_wgpu::WgpuConfiguration {
	egui_wgpu::WgpuConfiguration {
		power_preference: egui_wgpu::wgpu::PowerPreference::LowPower,
		..Default::default()
	}
}
