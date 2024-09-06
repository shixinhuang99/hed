use crate::Hed;

pub fn header(_hed: &mut Hed, ctx: &egui::Context) {
	egui::TopBottomPanel::top("header")
		.exact_height(40.0)
		.show(ctx, |_ui| {
			//
		});
}
