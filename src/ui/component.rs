use egui::{
	Align, Align2, FontId, Frame, Grid, Key, Layout, Margin, RichText,
	TextEdit, Ui, Window,
};

pub fn div(
	ui: &mut Ui,
	margin: impl Into<Margin>,
	child: impl FnOnce(&mut Ui),
) {
	Frame::none().inner_margin(margin).show(ui, child);
}

pub struct FormWindowResponse {
	pub close: bool,
	pub ok: bool,
}

pub fn form_window(
	ui: &mut Ui,
	title: &str,
	mut open: bool,
	child: impl FnOnce(&mut Ui),
) -> FormWindowResponse {
	let mut resp = FormWindowResponse {
		close: false,
		ok: false,
	};

	Window::new(title)
		.open(&mut open)
		.anchor(Align2::CENTER_CENTER, [0.0, 0.0])
		.collapsible(false)
		.resizable([false, false])
		.show(ui.ctx(), |ui| {
			Grid::new(format!("{}_grid", title))
				.num_columns(2)
				.striped(true)
				.min_col_width(60.0)
				.max_col_width(220.0)
				.min_row_height(40.0)
				.show(ui, child);
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
								.selectable_label(
									true,
									RichText::new("OK").size(16.0),
								)
								.clicked()
							{
								resp.ok = true;
							}
						},
					);
				});
			});
		});

	if !open || ui.input(|i| i.key_pressed(Key::Escape)) {
		resp.close = true;
	}

	resp
}

pub fn input<'a>(
	value: &'a mut String,
	placeholder: &'static str,
	full: bool,
) -> TextEdit<'a> {
	let mut te = TextEdit::singleline(value)
		.vertical_align(Align::Center)
		.margin(Margin::symmetric(6.0, 4.0))
		.hint_text(placeholder)
		.font(FontId::proportional(15.0));

	if full {
		te = te.desired_width(f32::INFINITY);
	}

	te
}

pub fn text_area<'a>(
	value: &'a mut String,
	placeholder: &'static str,
) -> TextEdit<'a> {
	TextEdit::multiline(value)
		.margin(Margin::symmetric(6.0, 4.0))
		.hint_text(placeholder)
		.font(FontId::proportional(15.0))
}
