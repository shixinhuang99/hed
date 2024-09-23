use egui::Ui;

use super::component::{form_window, input, text_area};
use crate::core::Hed;

pub fn new_item_window(ui: &mut Ui, hed: &mut Hed) {
	let resp = form_window(ui, "New item", hed.new_item_window_open, |ui| {
		ui.heading("ip: ");
		let ip_input = ui.add(input(&mut hed.item_form.ip, "ip", true));
		if !hed.item_form.ip_error.is_empty() {
			ip_input.show_tooltip_text(&hed.item_form.ip_error);
			if ip_input.gained_focus() {
				hed.item_form.ip_error.clear();
			}
		}
		ui.end_row();
		ui.heading("hosts: ");
		let hosts_input = ui
			.add(text_area(&mut hed.item_form.hosts, "hosts").desired_rows(8));
		if !hed.item_form.hosts_error.is_empty() {
			hosts_input.show_tooltip_text(&hed.item_form.hosts_error);
			if hosts_input.gained_focus() {
				hed.item_form.hosts_error.clear();
			}
		}
		ui.end_row();
	});

	if resp.close {
		hed.close_new_item_window();
	}

	if resp.ok {
		hed.new_item();
	}
}
