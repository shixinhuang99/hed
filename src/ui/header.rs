#[cfg(feature = "_dev")]
use egui::Window;
use egui::{
	special_emojis, Align, Context, FontId, Layout, RichText, TopBottomPanel,
	Ui, Visuals,
};

use super::common::set_button_padding;
use crate::{
	consts::{APP_NAME, APP_REPOSITORY, APP_VER},
	core::Hed,
};

pub fn header(ctx: &Context, _hed: &mut Hed) {
	TopBottomPanel::top("header")
		.exact_height(48.0)
		.show(ctx, |ui| {
			ui.horizontal_centered(|ui| {
				ui.heading(first_uppercase(APP_NAME));
				ui.label(
					RichText::new(APP_VER).font(FontId::proportional(14.0)),
				);
				ui.hyperlink_to(
					RichText::new(format!(
						"{} {} on GitHub",
						special_emojis::GITHUB,
						APP_NAME
					))
					.font(FontId::proportional(14.0)),
					APP_REPOSITORY,
				);
				ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
					set_button_padding(ui);

					#[cfg(feature = "_dev")]
					dev_window(ctx, ui, _hed);

					theme_switch(ctx, ui);
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

fn first_uppercase(s: &str) -> String {
	let mut s = s.to_string();
	if let Some(first) = s.get_mut(0..1) {
		first.make_ascii_uppercase();
	}
	s
}
