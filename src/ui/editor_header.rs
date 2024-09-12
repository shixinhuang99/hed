use egui::{
	Align, Button, Context, Label, Layout, RichText, TopBottomPanel, Ui,
};

use super::common::{
	pretty_btn_shortcut, reset_btn_shortcut, save_btn_shortcut,
	set_button_padding,
};
use crate::core::{EditorKind, Hed};

pub fn editor_header(ctx: &Context, hed: &mut Hed) {
	TopBottomPanel::top("editor_header")
		.exact_height(40.0)
		.show(ctx, |ui| {
			panel_content(ctx, ui, hed);
		});
}

fn panel_content(ctx: &Context, ui: &mut Ui, hed: &mut Hed) {
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

	let panel_width = ui.available_width();
	let panel_height = ui.available_height();

	let changed = profile.is_changed();

	ui.horizontal(|ui| {
		ui.allocate_ui_with_layout(
			[panel_width * 0.2, panel_height].into(),
			Layout::left_to_right(Align::Center),
			|ui| {
				ui.add(
					Label::new(RichText::new(&profile.name).heading())
						.truncate(),
				);
			},
		);

		set_button_padding(ui);

		if panel_width < 1200.0 {
			ui.add_space(10.0);
			ui.selectable_value(
				&mut hed.editor_kind,
				EditorKind::Options,
				"Options View",
			);
			ui.heading("/");
			ui.selectable_value(
				&mut hed.editor_kind,
				EditorKind::Text,
				"Text View",
			);
		}

		ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
			if ui
				.add_enabled(
					changed,
					Button::new("Reset").shortcut_text(
						ctx.format_shortcut(&reset_btn_shortcut()),
					),
				)
				.clicked()
			{
				profile.reset_content();
			}

			if ui
				.add_enabled(
					changed,
					Button::new("Save").shortcut_text(
						ctx.format_shortcut(&save_btn_shortcut()),
					),
				)
				.clicked()
			{
				profile.save_content();
			};

			if ui
				.add(
					Button::new("pretty").shortcut_text(
						ctx.format_shortcut(&pretty_btn_shortcut()),
					),
				)
				.clicked()
			{
				profile.pretty();
			}
		});
	});
}
