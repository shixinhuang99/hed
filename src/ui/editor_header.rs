use crate::Hed;

pub fn editor_header(_hed: &mut Hed, ctx: &egui::Context) {
	egui::TopBottomPanel::top("editor_header")
		.exact_height(40.0)
		.show(ctx, |_ui| {
			//
		});
}
