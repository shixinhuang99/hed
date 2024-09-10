use egui::{CentralPanel, Context, FontId, ScrollArea, TextEdit, Ui};

use super::common::{reset_btn_shortcut, save_btn_shortcut};
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

	let Some(profile) = hed.selected_profile_mut() else {
		return;
	};

	ScrollArea::vertical().show(ui, |ui| {
		ui.centered_and_justified(|ui| {
			let output = TextEdit::multiline(&mut profile.content_draft)
				.code_editor()
				.font(FontId::monospace(16.0))
				.show(ui);

			if output.response.has_focus() {
				if ui.input_mut(|i| i.consume_shortcut(&save_btn_shortcut())) {
					profile.save_content();
				}

				if ui.input_mut(|i| i.consume_shortcut(&reset_btn_shortcut())) {
					profile.reset_content();
				}
			}

			// TODO: text opreation, context menu, syntax highlight
		});
	});
}
