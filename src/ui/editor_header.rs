use crate::core::Hed;

pub fn editor_header(ctx: &egui::Context, _hed: &mut Hed) {
	egui::TopBottomPanel::top("editor_header")
		.exact_height(40.0)
		.show(ctx, |_ui| {
			//
		});
}
