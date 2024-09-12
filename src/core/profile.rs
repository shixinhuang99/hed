use super::HostsInfo;
use crate::static_global_id;

static_global_id!(2);

pub struct Profile {
	pub id: usize,
	pub name: String,
	pub hosts_info: HostsInfo,
	pub hosts_info_draft: HostsInfo,
}

impl Default for Profile {
	fn default() -> Self {
		Self {
			id: GLOBAL_ID.next(),
			name: String::new(),
			hosts_info: HostsInfo::default(),
			hosts_info_draft: HostsInfo::default(),
		}
	}
}

impl Profile {
	pub fn new(name: &str) -> Self {
		Self {
			id: GLOBAL_ID.next(),
			name: name.to_string(),
			..Default::default()
		}
	}

	pub fn new_system() -> Self {
		Self {
			id: 1,
			name: "System".to_string(),
			..Default::default()
		}
	}

	pub fn save_content(&mut self) {
		self.hosts_info.content = self.hosts_info_draft.content.clone();
	}

	pub fn reset_content(&mut self) {
		self.hosts_info_draft.content = self.hosts_info.content.clone();
	}

	pub fn is_changed(&self) -> bool {
		self.hosts_info_draft.content != self.hosts_info.content
	}

	pub fn update_by_content_change(&mut self) {
		self.hosts_info_draft.update_by_content_change();
	}

	pub fn pretty(&mut self) {
		self.hosts_info_draft.pretty();
	}
}
