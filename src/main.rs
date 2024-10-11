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
		Box::new(|cc| {
			#[cfg(target_os = "windows")]
			set_fonts(cc);
			set_font_style(cc);

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
	let logo_img_data = include_bytes!(".././assets/icon.png");

	egui::ViewportBuilder::default()
		.with_min_inner_size(min_size)
		.with_inner_size(min_size)
		.with_title(title)
		.with_icon(eframe::icon_data::from_png_bytes(logo_img_data).unwrap())
}

fn create_wgpu_options() -> egui_wgpu::WgpuConfiguration {
	egui_wgpu::WgpuConfiguration {
		power_preference: egui_wgpu::wgpu::PowerPreference::LowPower,
		..Default::default()
	}
}

#[cfg(target_os = "windows")]
fn set_fonts(cc: &eframe::CreationContext) {
	use std::fs;

	use egui::{FontData, FontDefinitions, FontFamily};

	let Ok(raw_font_data) = fs::read("C:\\Windows\\Fonts\\simhei.ttf") else {
		return;
	};

	let font_name = "simhei".to_string();
	let mut fonts = FontDefinitions::default();
	let font_data = FontData::from_owned(raw_font_data);

	fonts.font_data.insert(font_name.clone(), font_data);

	fonts
		.families
		.entry(FontFamily::Proportional)
		.or_default()
		.push(font_name.clone());

	fonts
		.families
		.entry(FontFamily::Monospace)
		.or_default()
		.push(font_name);

	cc.egui_ctx.set_fonts(fonts);
}

fn set_font_style(cc: &eframe::CreationContext) {
	use egui::TextStyle;

	cc.egui_ctx.style_mut(|style| {
		for (text_style, font) in &mut style.text_styles {
			if matches!(
				text_style,
				TextStyle::Body | TextStyle::Button | TextStyle::Monospace
			) {
				font.size = 16.0;
			}
		}
	});
}
