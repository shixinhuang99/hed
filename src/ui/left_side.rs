use egui::{
	style::Widgets, Context, Frame, Label, Margin, RichText, ScrollArea,
	SidePanel, TextStyle,
};

use crate::Hed;

pub fn left_side(ctx: &Context, _hed: &mut Hed) {
	let ctx_style = &ctx.style();
	let hovered_visuals = if ctx_style.visuals.dark_mode {
		Widgets::dark().hovered
	} else {
		Widgets::light().hovered
	};
	let noninteractive_visuals = if ctx_style.visuals.dark_mode {
		Widgets::dark().noninteractive
	} else {
		Widgets::light().noninteractive
	};
	let selection_visuals = ctx_style.visuals.selection;
	SidePanel::left("left_side")
		.exact_width(200.0)
		.resizable(false)
		.frame(Frame::side_top_panel(ctx_style).inner_margin(Margin {
			right: 2.0,
			..Default::default()
		}))
		.show(ctx, |ui| {
			let panel_width = ui.available_width();
			ScrollArea::vertical().show(ui, |ui| {
				ui.set_width(panel_width);
				for i in 0..100 {
					let selected = i == 0;
					Frame::none()
						.inner_margin(Margin {
							left: 2.0,
							right: 1.0,
							..Default::default()
						})
						.show(ui, |ui| {
							ui.set_height(60.0);
							let mut frame = Frame::none().begin(ui);
							let content_ui = &mut frame.content_ui;
							content_ui.set_height(ui.available_height());
							content_ui.set_width(ui.available_width());
							content_ui.horizontal_centered(|ui| {
								ui.add_space(12.0);
								ui.add(
									Label::new(
										RichText::new(format!("profile {}", i))
											.size(20.0)
											.text_style(TextStyle::Button)
											.color(if selected {
												hovered_visuals.text_color()
											} else {
												noninteractive_visuals
													.text_color()
											}),
									)
									.truncate(),
								);
							});
							let resp = frame.allocate_space(ui);
							if selected {
								frame.frame.rounding = hovered_visuals.rounding;
								frame.frame.fill = selection_visuals.bg_fill;
								if resp.hovered() {
									frame.frame.stroke =
										selection_visuals.stroke;
								}
							} else if resp.hovered() {
								frame.frame.rounding = hovered_visuals.rounding;
								frame.frame.stroke = hovered_visuals.bg_stroke;
								frame.frame.fill = hovered_visuals.weak_bg_fill;
							}
							frame.paint(ui);
						});
				}
			});
		});
}
