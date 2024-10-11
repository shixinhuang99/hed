use egui::Ui;

use super::component::{form_window, input, show_error_tooltip, text_area};
use crate::core::{Hed, OpenedWindow};

pub fn new_item_window(ui: &mut Ui, hed: &mut Hed) {
	if !hed.is_window_open(OpenedWindow::NewItem) {
		return;
	}

	let window = form_window(ui, "New item", |ui| {
		ui.heading("ip: ");
		let ip_input = ui.add(input(&mut hed.item_form.ip, "ip", true));
		show_error_tooltip(ip_input, &mut hed.item_form.ip_error);
		ui.end_row();
		ui.heading("hosts: ");
		let hosts_input = ui
			.add(text_area(&mut hed.item_form.hosts, "hosts").desired_rows(8));
		show_error_tooltip(hosts_input, &mut hed.item_form.hosts_error);
		ui.end_row();
	});

	if window.close {
		hed.close_item_form_window();
	}

	if window.ok {
		hed.new_item();
	}
}

pub fn add_hosts_window(ui: &mut Ui, hed: &mut Hed) {
	if !hed.is_window_open(OpenedWindow::AddHosts) {
		return;
	}

	let window = form_window(ui, "Add Hosts", |ui| {
		ui.heading("hosts: ");
		let hosts_input = ui
			.add(text_area(&mut hed.item_form.hosts, "hosts").desired_rows(8));
		show_error_tooltip(hosts_input, &mut hed.item_form.hosts_error);
		ui.end_row();
	});

	if window.close {
		hed.close_add_hosts_window();
	}

	if window.ok {
		hed.add_hosts();
	}
}

pub fn edit_host_window(ui: &mut Ui, hed: &mut Hed) {
	if !hed.is_window_open(OpenedWindow::EditHost) {
		return;
	}

	let window = form_window(ui, "Edit Host", |ui| {
		ui.heading("hosts: ");
		let hosts_input =
			ui.add(input(&mut hed.item_form.hosts, "hosts", true));
		show_error_tooltip(hosts_input, &mut hed.item_form.hosts_error);
		ui.end_row();
	});

	if window.close {
		hed.close_edit_host_window();
	}

	if window.ok {
		hed.edit_host();
	}
}
