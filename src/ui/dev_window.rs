use crate::Hed;

pub fn dev_window(ctx: &egui::Context, ui: &mut egui::Ui, hed: &mut Hed) {
	if ui.button("dev_window").clicked() {
		hed.dev_window_open = !hed.dev_window_open;
	}
	egui::Window::new("dev_window")
		.open(&mut hed.dev_window_open)
		.vscroll(true)
		.show(ctx, |ui| {
			ctx.style_ui(ui);
		});
}
