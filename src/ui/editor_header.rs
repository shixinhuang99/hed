use egui::{Align, Button, Context, Layout, TopBottomPanel, Ui};

use super::common::{
	reset_btn_shortcut, save_btn_shortcut, set_button_padding,
};
use crate::core::{Hed, ViewKind};

pub fn editor_header(ctx: &Context, hed: &mut Hed) {
	if hed.sys_hosts_loading || !hed.parse_sys_hosts_err.is_empty() {
		return;
	}

	TopBottomPanel::top("editor_header")
		.exact_height(40.0)
		.show(ctx, |ui| {
			ui.add_enabled_ui(hed.opened_window.is_none(), |ui| {
				panel_content(ui, hed);
			});
		});
}

fn panel_content(ui: &mut Ui, hed: &mut Hed) {
	let panel_width = ui.available_width();
	let changed = hed.is_hosts_changed();

	ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
		set_button_padding(ui);

		if panel_width < 1200.0 {
			hed.view_all = false;
			ui.selectable_value(
				&mut hed.view_kind,
				ViewKind::Options,
				"Options View",
			);
			ui.heading("/");
			ui.selectable_value(
				&mut hed.view_kind,
				ViewKind::Text,
				"Text View",
			);
		} else {
			hed.view_all = true;
		}

		ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
			if ui
				.add_enabled(
					changed,
					Button::new("Reset").shortcut_text(
						ui.ctx().format_shortcut(&reset_btn_shortcut()),
					),
				)
				.clicked()
			{
				hed.reset_hosts();
			}

			if ui
				.add_enabled(
					changed,
					Button::new("Save").shortcut_text(
						ui.ctx().format_shortcut(&save_btn_shortcut()),
					),
				)
				.clicked()
			{
				hed.save_hosts();
			};
		});
	});
}
