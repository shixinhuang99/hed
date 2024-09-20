use egui::{Key, Ui};

use super::component::{form_window, input};
use crate::core::Hed;

pub fn new_profile_window(ui: &mut Ui, hed: &mut Hed) {
	let title = "+ New Profile";

	if ui.button(title).clicked() {
		hed.new_profile_window_open = true;
	}

	let resp = form_window(ui, title, hed.new_profile_window_open, |ui| {
		ui.heading("name: ");
		let resp = ui.add(input(
			&mut hed.new_profile_form.name,
			"new profile name",
			true,
		));
		if resp.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
			hed.create_profile();
		}
		if !hed.new_profile_form.error.is_empty() {
			resp.show_tooltip_text(&hed.new_profile_form.error);
			if resp.gained_focus() {
				hed.new_profile_form.error.clear();
			}
		}
		ui.end_row();
	});

	if resp.close {
		hed.close_new_profile_window();
	}

	if resp.ok {
		hed.create_profile();
	}
}
