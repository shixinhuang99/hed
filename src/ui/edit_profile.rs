use egui::{Key, Ui};

use super::component::{form_window, input};
use crate::core::Hed;

pub fn edit_profile_window(ui: &mut Ui, hed: &mut Hed) {
	if hed.edit_profile_id.is_none() {
		return;
	}

	let resp = form_window(ui, "Edit Profile", true, |ui| {
		ui.heading("name: ");
		let resp = ui.add(input(
			&mut hed.edit_profile_form.name,
			"new profile name",
			true,
		));
		if resp.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
			hed.edit_profile();
		}
		if !hed.edit_profile_form.error.is_empty() {
			resp.show_tooltip_text(&hed.edit_profile_form.error);
			if resp.gained_focus() {
				hed.edit_profile_form.error.clear();
			}
		}
		ui.end_row();
	});

	if resp.close {
		hed.close_edit_profile_window();
	}

	if resp.ok {
		hed.edit_profile();
	}
}
