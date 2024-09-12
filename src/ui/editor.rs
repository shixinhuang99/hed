use egui::{
	Align, CentralPanel, Context, FontId, Frame, Margin, ScrollArea, TextEdit,
	Ui,
};

use super::common::{
	pretty_btn_shortcut, reset_btn_shortcut, save_btn_shortcut,
	set_button_padding,
};
use crate::core::{EditorKind, Hed};

pub fn editor(ctx: &Context, hed: &mut Hed) {
	CentralPanel::default().show(ctx, |ui| {
		panel_content(ui, hed);
	});
}

fn panel_content(ui: &mut Ui, hed: &mut Hed) {
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

	let mut options_view = || {
		ui.scope(|ui| {
			set_button_padding(ui);

			ui.horizontal(|ui| {
				ui.set_height(30.0);
				if ui.button("+ Add item").clicked() {
					//
				}
				ui.add(
					TextEdit::singleline(&mut hed.search_ip_hosts)
						.desired_width(f32::INFINITY)
						.font(FontId::monospace(16.0))
						.vertical_align(Align::Center)
						.margin(Margin::symmetric(6.0, 4.0))
						.hint_text("Search ip, hosts"),
				);
			});

			ui.separator();

			ScrollArea::vertical().show(ui, |ui| {
				for ip_hosts in &mut profile.hosts_info_draft.list {
					if !ip_hosts.contains(&hed.search_ip_hosts) {
						continue;
					}
					Frame::none()
						.inner_margin(Margin {
							right: 8.0,
							..Default::default()
						})
						.show(ui, |ui| {
							ui.horizontal(|ui| {
								ui.vertical(|ui| {
									ui.add_space(8.0);
									ui.horizontal(|ui| {
										if ui.button("⛭").clicked() {
											//
										}
										ui.add(
											TextEdit::singleline(
												&mut ip_hosts.ip,
											)
											.font(FontId::monospace(16.0))
											.vertical_align(Align::Center)
											.margin(Margin::symmetric(
												6.0, 4.0,
											)),
										);
									});
								});
								ui.horizontal_wrapped(|ui| {
									for host in &ip_hosts.enabled_hosts {
										if ui
											.selectable_label(true, host)
											.clicked()
										{
											//
										}
									}
									for host in &ip_hosts.disabled_hosts {
										if ui
											.selectable_label(false, host)
											.clicked()
										{
											//
										}
									}
								});
							});
						});
					ui.separator();
				}
			});
		});
	};

	match hed.editor_kind {
		EditorKind::Options => {
			options_view();
		}
		EditorKind::Text => {
			ScrollArea::vertical().show(ui, |ui| {
				ui.centered_and_justified(|ui| {
					let output = TextEdit::multiline(
						&mut profile.hosts_info_draft.content,
					)
					.code_editor()
					.font(FontId::monospace(16.0))
					.show(ui);

					if output.response.has_focus() {
						if ui.input_mut(|i| {
							i.consume_shortcut(&save_btn_shortcut())
						}) {
							profile.save_content();
						}

						if ui.input_mut(|i| {
							i.consume_shortcut(&reset_btn_shortcut())
						}) {
							profile.reset_content();
						}

						if ui.input_mut(|i| {
							i.consume_shortcut(&pretty_btn_shortcut())
						}) {
							profile.pretty();
						}
					}

					if output.response.changed() {
						profile.update_by_content_change();
					}

					// TODO: text opreation, context menu, syntax highlight
				});
			});
		}
	}
}
