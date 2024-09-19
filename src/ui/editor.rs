use egui::{
	Button, CentralPanel, Context, FontId, Margin, ScrollArea, SelectableLabel,
	TextEdit, Ui,
};

use super::{
	common::{
		pretty_btn_shortcut, reset_btn_shortcut, save_btn_shortcut,
		set_button_padding,
	},
	component::{div, form_window, input, text_area},
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
				if ui.button("+ New item").clicked() {
					hed.new_item_window_open = true;
				}
				ui.add(input(
					&mut hed.search_ip_hosts,
					"Search ip, hosts",
					true,
				));
			});

			ui.separator();

			ScrollArea::vertical().show(ui, |ui| {
				for ip_hosts in &mut profile.hosts_info_draft.list {
					if !ip_hosts.contains(&hed.search_ip_hosts) {
						continue;
					}
					div(
						ui,
						Margin {
							right: 8.0,
							..Default::default()
						},
						|ui| {
							ui.horizontal(|ui| {
								ui.vertical(|ui| {
									ui.add_space(8.0);
									ui.horizontal(|ui| {
										ui.menu_button("⛭", |ui| {
											if ui.button("Add host").clicked() {
												//
											}
											if ui.button("Enable").clicked() {
												//
											}
											if ui.button("Delete").clicked() {
												//
											}
										});
										ui.add(input(
											&mut ip_hosts.ip,
											"ip",
											false,
										));
									});
								});
								ui.horizontal_wrapped(|ui| {
									let mut to_disabled_hosts = vec![];
									for host in &ip_hosts.enabled_hosts {
										let resp = ui.add(
											SelectableLabel::new(true, host),
										);
										if resp.clicked() {
											to_disabled_hosts
												.push(host.clone());
										}
										resp.context_menu(|ui| {
											if ui.button("Edit").clicked() {
												//
											}
											if ui.button("Delete").clicked() {
												//
											}
										});
									}
									for host in to_disabled_hosts {
										if ip_hosts
											.enabled_hosts
											.shift_remove(&host)
										{
											ip_hosts
												.disabled_hosts
												.insert(host);
										}
									}
									let mut to_enabled_hosts = vec![];
									for host in &ip_hosts.disabled_hosts {
										let resp = ui.add(Button::new(host));
										if resp.clicked() {
											to_enabled_hosts.push(host.clone());
										}
										resp.context_menu(|ui| {
											if ui.button("Edit").clicked() {
												//
											}
											if ui.button("Delete").clicked() {
												//
											}
										});
									}
									for host in to_enabled_hosts {
										if ip_hosts
											.disabled_hosts
											.shift_remove(&host)
										{
											ip_hosts.enabled_hosts.insert(host);
										}
									}
								});
							});
						},
					);

					ui.separator();
				}
			});
		});

		let resp =
			form_window(ui, "New item", hed.new_item_window_open, |ui| {
				ui.heading("ip: ");
				ui.add(input(&mut hed.new_item_ip, "ip", true));
				ui.end_row();
				ui.heading("hosts: ");
				ui.add(text_area(&mut hed.new_item_hosts, "hosts"));
				ui.end_row();
			});
		if resp.close {
			// hed.close_new_item_window();
		}
		if resp.ok {
			// hed.new_item();
		}
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
