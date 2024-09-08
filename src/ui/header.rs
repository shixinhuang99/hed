use crate::core::Hed;
#[cfg(feature = "_dev")]
use crate::ui::dev_window::dev_window;

use super::new_profile::new_profile_window;

pub fn header(ctx: &egui::Context, hed: &mut Hed) {
	let dark_mode = ctx.style().visuals.dark_mode;
	egui::TopBottomPanel::top("header")
		.exact_height(48.0)
		.show(ctx, |ui| {
			ui.horizontal_centered(|ui| {
				ui.heading("Hed");
				ui.with_layout(
					egui::Layout::right_to_left(egui::Align::Center),
					|ui| {
						ui.spacing_mut().button_padding.y = 6.0;

						#[cfg(feature = "_dev")]
						dev_window(ctx, ui, hed);

						if ui.selectable_label(!dark_mode, "☀ Light").clicked()
						{
							ctx.set_visuals(egui::Visuals::light());
						}
						if ui.selectable_label(dark_mode, "🌙 Dark").clicked()
						{
							ctx.set_visuals(egui::Visuals::dark());
						}

						new_profile_window(ctx, ui, hed);
					},
				);
			});
		});
}
