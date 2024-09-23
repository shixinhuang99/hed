use crate::util::{is_ip, StringExt};

#[derive(Default)]
pub struct ItemForm {
	pub ip: String,
	pub hosts: String,
	pub ip_error: String,
	pub hosts_error: String,
}

impl ItemForm {
	pub fn reset(&mut self) {
		self.ip.clear();
		self.hosts.clear();
		self.ip_error.clear();
		self.hosts_error.clear();
	}

	pub fn validate(&mut self) -> bool {
		self.ip.make_trim();
		if self.ip.is_empty() {
			self.ip_error = "IP address is empty".to_string();
			return false;
		}
		if !is_ip(&self.ip) {
			self.ip_error = format!("`{}` is not a valid IP address", self.ip);
			return false;
		}
		self.hosts.make_trim();
		if self.hosts.is_empty() {
			self.hosts_error = "hosts is empty".to_string();
			return false;
		}
		true
	}
}
