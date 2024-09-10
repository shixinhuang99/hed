use super::HostsInfo;
use crate::static_global_id;

static_global_id!(2);

pub struct Profile {
	pub id: usize,
	pub name: String,
	pub hosts_info: HostsInfo,
	pub content_draft: String,
}

impl Default for Profile {
	fn default() -> Self {
		Self {
			id: GLOBAL_ID.next(),
			name: String::new(),
			hosts_info: HostsInfo::default(),
			content_draft: String::new(),
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
		self.hosts_info.content = self.content_draft.clone();
	}

	pub fn reset_content(&mut self) {
		self.content_draft = self.hosts_info.content.clone();
	}

	pub fn is_changed(&self) -> bool {
		self.content_draft != self.hosts_info.content
	}
}
