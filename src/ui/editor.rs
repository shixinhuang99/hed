use egui::{Button, CentralPanel, Context, Margin, ScrollArea, TextEdit, Ui};

use super::{
	all_window::{add_hosts_window, edit_host_window, new_item_window},
	common::{
		format_btn_shortcut, reset_btn_shortcut, save_btn_shortcut,
		set_button_padding,
	},
	component::{div, input},
};
use crate::core::{Event, Hed, OpenedWindow, ViewKind};

pub fn editor(ctx: &Context, hed: &mut Hed) {
	CentralPanel::default().show(ctx, |ui| {
		if !hed.os_err.is_empty() {
			ui.centered_and_justified(|ui| {
				ui.heading(&hed.os_err);
			});
			return;
		}
		ui.add_enabled_ui(
			!hed.sys_hosts_loading && hed.opened_window.is_none(),
			|ui| {
				panel_content(ui, hed);
			},
		);
	});
}

fn panel_content(ui: &mut Ui, hed: &mut Hed) {
	set_button_padding(ui);

	if hed.view_all {
		let width = ui.available_width() - 20.0;
		ui.horizontal_centered(|ui| {
			ui.vertical(|ui| {
				ui.set_width(width / 2.0);
				options_view(ui, hed);
			});
			ui.separator();
			ui.vertical(|ui| {
				ui.set_width(width / 2.0);
				text_view(ui, hed);
			});
		});
		return;
	}

	match hed.view_kind {
		ViewKind::Options => {
			options_view(ui, hed);
		}
		ViewKind::Text => {
			text_view(ui, hed);
		}
	}
}

fn options_view(ui: &mut Ui, hed: &mut Hed) {
	ui.horizontal(|ui| {
		ui.set_height(30.0);
		if ui.button("+ New item").clicked() {
			hed.set_opened_window(OpenedWindow::NewItem);
		}
		ui.add(input(&mut hed.search_ip_hosts, "Search ip, hosts", true));
	});

	ui.separator();

	ScrollArea::vertical()
		.id_source("options_view")
		.show(ui, |ui| {
			div(
				ui,
				Margin {
					right: 8.0,
					..Default::default()
				},
				|ui| {
					for item in &hed.hosts_info_draft.list {
						if !item.contains(&hed.search_ip_hosts) {
							continue;
						}
						ui.horizontal(|ui| {
							ui.vertical(|ui| {
								ui.add_space(8.0);
								ui.horizontal(|ui| {
									ui.menu_button("⛭", |ui| {
										if ui.button("Add hosts").clicked() {
											hed.send_event(
												Event::OpenAddHostsWindow(
													item.id,
												),
											);
											ui.close_menu();
										}
										if ui.button("Delete").clicked() {
											hed.send_event(Event::DeleteItem(
												item.id,
											));
											ui.close_menu();
										}
									});
									let mut ip = item.ip.clone();
									let input =
										ui.add(input(&mut ip, "ip", false));
									if input.changed() {
										hed.send_event(Event::EditItemIp(
											item.id, ip,
										));
									}
								});
							});
							ui.horizontal_wrapped(|ui| {
								for host in &item.hosts {
									let btn = if host.enabled {
										ui.selectable_label(true, &host.name)
									} else {
										ui.button(&host.name)
									};
									if btn.clicked() {
										hed.send_event(
											Event::ToggleHostEnable(
												item.id, host.id,
											),
										);
									}
									btn.context_menu(|ui| {
										set_button_padding(ui);
										if ui.button("Edit").clicked() {
											hed.send_event(
												Event::OpenEditHostWindow(
													item.id, host.id,
												),
											);
											ui.close_menu();
										}
										if ui.button("Delete").clicked() {
											hed.send_event(Event::DeleteHost(
												item.id, host.id,
											));
											ui.close_menu();
										}
									});
								}
							});
						});
						ui.separator();
					}
				},
			);
		});

	new_item_window(ui, hed);
	add_hosts_window(ui, hed);
	edit_host_window(ui, hed);
}

fn text_view(ui: &mut Ui, hed: &mut Hed) {
	ui.horizontal(|ui| {
		ui.set_height(30.0);
		if ui
			.add(Button::new("Fromat").shortcut_text(
				ui.ctx().format_shortcut(&format_btn_shortcut()),
			))
			.clicked()
		{
			hed.update_content();
		}
	});

	ui.separator();

	ScrollArea::vertical()
		.id_source("text_view")
		.show(ui, |ui| {
			ui.centered_and_justified(|ui| {
				let output =
					TextEdit::multiline(&mut hed.hosts_info_draft.content)
						.code_editor()
						.show(ui);

				if output.response.has_focus() {
					if ui
						.input_mut(|i| i.consume_shortcut(&save_btn_shortcut()))
					{
						hed.save_hosts();
					}

					if ui.input_mut(|i| {
						i.consume_shortcut(&reset_btn_shortcut())
					}) {
						hed.reset_hosts();
					}

					if ui.input_mut(|i| {
						i.consume_shortcut(&format_btn_shortcut())
					}) {
						hed.update_content();
					}
				}

				if output.response.changed() {
					hed.update_list();
				}
			});
		});
}
