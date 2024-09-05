use super::HostsInfo;
use crate::static_global_id;

static_global_id!(2);

pub struct Profile {
	pub id: usize,
	pub name: String,
	pub hosts_info: HostsInfo,
	pub deletable: bool,
}

impl Profile {
	pub fn new(name: &str) -> Self {
		Self {
			id: GLOBAL_ID.next(),
			name: name.to_string(),
			hosts_info: HostsInfo::default(),
			deletable: true,
		}
	}

	pub fn new_system() -> Self {
		Self {
			id: 1,
			name: "System".to_string(),
			hosts_info: HostsInfo::default(),
			deletable: false,
		}
	}
}
