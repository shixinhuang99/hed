#[cfg(feature = "_dev")]
use crate::ui::dev_window::dev_window;
use crate::Hed;

pub fn header(ctx: &egui::Context, _hed: &mut Hed) {
	egui::TopBottomPanel::top("header")
		.exact_height(40.0)
		.show(ctx, |ui| {
			ui.horizontal_centered(|ui| {
				ui.heading("Hed");
				ui.with_layout(
					egui::Layout::right_to_left(egui::Align::Center),
					|ui| {
						#[cfg(feature = "_dev")]
						dev_window(ctx, ui, _hed);
						egui::global_dark_light_mode_buttons(ui);
					},
				);
			});
		});
}
