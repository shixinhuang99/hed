use egui::{CentralPanel, Context, Ui};

use crate::core::Hed;

pub fn editor(ctx: &Context, hed: &mut Hed) {
	CentralPanel::default().show(ctx, |ui| {
		panel_content(ui, hed);
	});
}

fn panel_content(ui: &mut Ui, hed: &mut Hed) {
	if hed.profiles_loading {
		return;
	}

	let Some(profile) = hed
		.profiles
		.iter_mut()
		.find(|p| p.id == hed.selected_profile_id)
	else {
		return;
	};

	ui.text_edit_multiline(&mut profile.hosts_info.content);
}
