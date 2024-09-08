use egui::{
	Button, Context, Frame, Margin, RichText, ScrollArea, SidePanel, TextEdit,
};

use super::widgets::ProfileLabel;
use crate::core::Hed;

pub fn left_side(ctx: &Context, hed: &mut Hed) {
	let ctx_style = &ctx.style();
	hed.check_profile_deleted();
	SidePanel::left("left_side")
		.width_range(200.0..=400.0)
		.resizable(true)
		.frame(Frame::side_top_panel(ctx_style).inner_margin(Margin {
			right: 4.0,
			..Default::default()
		}))
		.show(ctx, |ui| {
			let panel_width = ui.available_width();
			Frame::none().inner_margin(4.0).show(ui, |ui| {
				ui.set_height(30.0);
				ui.add(
					TextEdit::singleline(&mut hed.search_profile)
						.desired_width(f32::INFINITY)
						.font(egui::FontId::monospace(20.0))
						.vertical_align(egui::Align::Center)
						.hint_text("Search Profile"),
				);
			});
			ScrollArea::vertical().show(ui, |ui| {
				ui.set_width(panel_width);
				for profile in hed
					.profiles
					.iter()
					.filter(|p| p.name.contains(hed.search_profile.trim()))
				{
					let selected = profile.id == hed.selected_profile_id;
					let enalebd = profile.id == hed.enabled_profile_id;
					Frame::none()
						.inner_margin(Margin {
							left: 3.0,
							..Default::default()
						})
						.show(ui, |ui| {
							ui.spacing_mut().button_padding.x = 20.0;
							let resp = ui.add(ProfileLabel::new(
								selected,
								enalebd,
								RichText::new(&profile.name).size(20.0),
								60.0,
							));
							if resp.clicked() {
								hed.selected_profile_id = profile.id;
							}
							resp.on_hover_ui(|ui| {
								ui.style_mut().interaction.selectable_labels =
									true;
								let mut text = profile.name.to_string();
								if enalebd {
									text.push_str("\n(enabled)");
								}
								ui.label(text);
							})
							.context_menu(|ui| {
								ui.spacing_mut().button_padding.y = 8.0;
								if ui
									.add_enabled(
										profile.id != hed.enabled_profile_id,
										Button::new("Enable This Profile"),
									)
									.clicked()
								{
									hed.enabled_profile_id = profile.id;
									ui.close_menu();
								}
								if ui
									.add_enabled(
										hed.profiles.len() > 1
											&& profile.id
												!= hed.enabled_profile_id,
										Button::new("Delete"),
									)
									.clicked()
								{
									hed.mark_deleted_profile_id =
										Some(profile.id);
									ui.close_menu();
								};
							});
						});
				}
			});
		});
}
