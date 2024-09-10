use egui::{Align, Context, Layout, TopBottomPanel, Ui, Visuals, Window};

use super::{common::set_button_padding, new_profile::new_profile_window};
use crate::core::Hed;

pub fn header(ctx: &Context, hed: &mut Hed) {
	TopBottomPanel::top("header")
		.exact_height(48.0)
		.show(ctx, |ui| {
			ui.horizontal_centered(|ui| {
				ui.heading("Hed");
				ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
					set_button_padding(ui);

					#[cfg(feature = "_dev")]
					dev_window(ctx, ui, hed);

					theme_switch(ctx, ui);
					new_profile_window(ctx, ui, hed);
				});
			});
		});
}

#[cfg(feature = "_dev")]
fn dev_window(ctx: &Context, ui: &mut Ui, hed: &mut Hed) {
	if ui.button("dev_window").clicked() {
		hed.dev_window_open = !hed.dev_window_open;
	}
	Window::new("dev_window")
		.open(&mut hed.dev_window_open)
		.vscroll(true)
		.show(ctx, |ui| {
			ctx.style_ui(ui);
		});
}

fn theme_switch(ctx: &Context, ui: &mut Ui) {
	let dark_mode = ctx.style().visuals.dark_mode;
	if ui.selectable_label(!dark_mode, "☀ Light").clicked() {
		ctx.set_visuals(Visuals::light());
	}
	if ui.selectable_label(dark_mode, "🌙 Dark").clicked() {
		ctx.set_visuals(Visuals::dark());
	}
}
