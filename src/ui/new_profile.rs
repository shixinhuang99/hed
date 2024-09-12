use egui::{
	Align, Align2, Context, FontId, Frame, Grid, Key, Layout, RichText,
	TextEdit, Ui, Window,
};

use crate::core::Hed;

pub fn new_profile_window(ctx: &Context, ui: &mut Ui, hed: &mut Hed) {
	let title = "⊞ New Profile";

	if ui.selectable_label(true, title).clicked() {
		hed.new_profile_window_open = true;
	}
	if ui.input(|i| i.key_pressed(Key::Escape)) {
		hed.close_new_profile_window();
	}

	let mut open = hed.new_profile_window_open;
	Window::new(title)
		.id("new_profile".into())
		.open(&mut open)
		.anchor(Align2::CENTER_CENTER, [0.0, 0.0])
		.collapsible(false)
		.resizable([false, false])
		.show(ctx, |ui| {
			window_content(ui, hed);
		});
	if !open {
		hed.close_new_profile_window();
	}
}

fn window_content(ui: &mut Ui, hed: &mut Hed) {
	Grid::new("new_profile_form")
		.num_columns(2)
		.striped(true)
		.min_col_width(60.0)
		.max_col_width(220.0)
		.min_row_height(40.0)
		.show(ui, |ui| {
			form(ui, hed);
		});
	form_footer(ui, hed);
}

fn form(ui: &mut Ui, hed: &mut Hed) {
	ui.heading("name: ");
	let input_resp = ui.add(
		TextEdit::singleline(&mut hed.new_profile_name)
			.desired_width(f32::INFINITY)
			.font(FontId::proportional(18.0))
			.vertical_align(Align::Center)
			.hint_text("New profile name"),
	);
	if input_resp.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
		hed.create_profile();
	}
	if !hed.new_profile_err.is_empty() {
		input_resp.show_tooltip_text(&hed.new_profile_err);
		if input_resp.gained_focus() {
			hed.new_profile_err.clear();
		}
	}
	ui.end_row();
}

fn form_footer(ui: &mut Ui, hed: &mut Hed) {
	ui.scope(|ui| {
		ui.set_width(290.0);
		ui.separator();
		Frame::none().inner_margin(6.0).show(ui, |ui| {
			ui.with_layout(
				Layout::default()
					.with_main_wrap(false)
					.with_cross_align(Align::Center)
					.with_cross_justify(true),
				|ui| {
					if ui
						.selectable_label(true, RichText::new("OK").size(16.0))
						.clicked()
					{
						hed.create_profile();
					}
				},
			);
		});
	});
}
