use egui::{Align, Context, Label, Layout, RichText, TopBottomPanel, Ui};

use crate::core::Hed;

pub fn editor_header(ctx: &Context, hed: &mut Hed) {
	TopBottomPanel::top("editor_header")
		.exact_height(40.0)
		.show(ctx, |ui| {
			panel_content(ui, hed);
		});
}

fn panel_content(ui: &mut Ui, hed: &mut Hed) {
	if hed.profiles_loading {
		return;
	}

	let Some(profile) = hed
		.profiles
		.iter()
		.find(|p| p.id == hed.selected_profile_id)
	else {
		return;
	};

	let panel_width = ui.available_width();
	let panel_height = ui.available_height();

	ui.allocate_ui_with_layout(
		[panel_width * 0.2, panel_height].into(),
		Layout::left_to_right(Align::Center),
		|ui| {
			ui.add(
				Label::new(RichText::new(&profile.name).heading()).truncate(),
			);
		},
	);
}
