use egui::{
	Align, Button, Context, Label, Layout, RichText, TopBottomPanel, Ui,
};

use super::common::{
	pretty_btn_shortcut, reset_btn_shortcut, save_btn_shortcut,
	set_button_padding,
};
use crate::core::{Event, Hed};

pub fn editor_header(ctx: &Context, hed: &mut Hed) {
	TopBottomPanel::top("editor_header")
		.exact_height(40.0)
		.show(ctx, |ui| {
			panel_content(ctx, ui, hed);
		});
}

fn panel_content(ctx: &Context, ui: &mut Ui, hed: &Hed) {
	if hed.profiles_loading {
		return;
	}

	let Some(profile) = hed.get_selected_profile() else {
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
			let options_view =
				ui.selectable_label(hed.view_kind.is_options(), "Options View");
			ui.heading("/");
			let text_view =
				ui.selectable_label(hed.view_kind.is_text(), "Text View");
			if options_view.clicked() || text_view.clicked() {
				hed.send_event(Event::ToggleViewKind);
			}
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
				hed.send_event(Event::ResetProfile);
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
				hed.send_event(Event::SavePorfile);
			};

			if ui
				.add(
					Button::new("pretty").shortcut_text(
						ctx.format_shortcut(&pretty_btn_shortcut()),
					),
				)
				.clicked()
			{
				// profile.pretty();
			}
		});
	});
}
