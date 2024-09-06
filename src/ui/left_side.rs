use crate::Hed;

pub fn left_side(_hed: &mut Hed, ctx: &egui::Context) {
	egui::SidePanel::left("left_side")
		.exact_width(200.0)
		.resizable(false)
		.show(ctx, |_ui| {
			//
		});
}
