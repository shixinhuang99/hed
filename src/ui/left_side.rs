use crate::Hed;

pub fn left_side(ctx: &egui::Context, _hed: &mut Hed) {
	egui::SidePanel::left("left_side")
		.exact_width(200.0)
		.resizable(false)
		.show(ctx, |_ui| {
			//
		});
}
