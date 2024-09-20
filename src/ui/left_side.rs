use egui::{
	Button, Context, Frame, Margin, RichText, ScrollArea, SidePanel, Spinner,
	Ui,
};

use super::{
	common::set_button_padding,
	component::{div, input},
	edit_profile::edit_profile_window,
	widgets::ProfileLabel,
};
use crate::core::{Event, Hed};

pub fn left_side(ctx: &Context, hed: &mut Hed) {
	SidePanel::left("left_side")
		.width_range(200.0..=400.0)
		.resizable(true)
		.frame(Frame::side_top_panel(&ctx.style()).inner_margin(Margin {
			right: 4.0,
			..Default::default()
		}))
		.show(ctx, |ui| {
			panel_content(ui, hed);
		});
}

fn panel_content(ui: &mut Ui, hed: &mut Hed) {
	if hed.profiles_loading {
		ui.centered_and_justified(|ui| {
			ui.add(Spinner::new().size(30.0));
		});
		return;
	}
	search_input(ui, hed);
	profile_list(ui, hed);
	edit_profile_window(ui, hed);
}

fn search_input(ui: &mut Ui, hed: &mut Hed) {
	div(ui, 4.0, |ui| {
		ui.set_height(30.0);
		ui.add(input(&mut hed.search_profile, "Search profile", true));
	});
}

fn profile_list(ui: &mut Ui, hed: &Hed) {
	let panel_width = ui.available_width();
	let at_least_one = hed.profiles.len() > 1;

	ScrollArea::vertical().show(ui, |ui| {
		ui.set_width(panel_width);
		for profile in hed.get_display_profiles() {
			let selected = profile.id == hed.selected_profile_id;
			let enabled = profile.id == hed.enabled_profile_id;
			div(
				ui,
				Margin {
					left: 3.0,
					..Default::default()
				},
				|ui| {
					ui.spacing_mut().button_padding.x = 20.0;
					let resp = ui.add(ProfileLabel::new(
						selected,
						enabled,
						RichText::new(&profile.name).size(20.0),
						60.0,
					));
					if resp.clicked() {
						hed.send_event(Event::SelectProfile(profile.id));
					}
					resp.on_hover_ui(|ui| {
						ui.style_mut().interaction.selectable_labels = true;
						let mut text = profile.name.clone();
						if enabled {
							text.push_str("\n(enabled)");
						}
						ui.label(text);
					})
					.context_menu(|ui| {
						set_button_padding(ui);
						if ui
							.add_enabled(!enabled, Button::new("Enable"))
							.clicked()
						{
							hed.send_event(Event::EnableProfile(profile.id));
							ui.close_menu();
						}
						if ui.button("Rename").clicked() {
							hed.send_event(Event::EditProfile(profile.id));
							ui.close_menu();
						}
						if ui
							.add_enabled(
								at_least_one && !enabled,
								Button::new("Delete"),
							)
							.clicked()
						{
							hed.send_event(Event::RemoveProfile(profile.id));
							ui.close_menu();
						};
					});
				},
			);
		}
	});
}
