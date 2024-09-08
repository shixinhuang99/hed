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
			let mut hed = Hed::new()?;

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
	const MIN_SIZE: [f32; 2] = [900.0, 600.0];
	const APP_TITLE: &str = "Hed";

	egui::ViewportBuilder::default()
		.with_min_inner_size(MIN_SIZE)
		.with_inner_size(MIN_SIZE)
		.with_title(APP_TITLE)
}

fn create_wgpu_options() -> egui_wgpu::WgpuConfiguration {
	egui_wgpu::WgpuConfiguration {
		power_preference: egui_wgpu::wgpu::PowerPreference::LowPower,
		..Default::default()
	}
}
