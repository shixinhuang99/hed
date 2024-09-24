use std::collections::HashSet;

use crate::util::{is_ip, GLOBAL_ID};

#[derive(Debug, Clone)]
pub struct Item {
	pub id: usize,
	pub ip: String,
	pub hosts: Vec<Host>,
}

#[derive(Debug, Clone)]
pub struct Host {
	pub id: usize,
	pub name: String,
	pub enabled: bool,
}

impl Item {
	pub fn new(ip: &str, hosts: Vec<String>, enabled: bool) -> Self {
		let mut item = Self {
			id: GLOBAL_ID.next(),
			ip: ip.to_string(),
			hosts: hosts
				.into_iter()
				.map(|name| Host {
					id: GLOBAL_ID.next(),
					name,
					enabled,
				})
				.collect(),
		};

		item.hosts_dedup();

		item
	}

	fn hosts_dedup(&mut self) {
		let mut set: HashSet<&str> = HashSet::new();
		let mut new_hosts = vec![];

		for host in &self.hosts {
			if !set.contains(host.name.as_str()) {
				new_hosts.push(host.clone());
				set.insert(&host.name);
			}
		}

		self.hosts = new_hosts;
	}

	pub fn add(&mut self, hosts: Vec<String>, enabled: bool) {
		for name in hosts {
			self.hosts.push(Host {
				id: GLOBAL_ID.next(),
				name,
				enabled,
			});
		}

		self.hosts_dedup();
	}

	pub fn contains(&self, s: &str) -> bool {
		self.ip.contains(s)
			|| self.hosts.iter().any(|host| host.name.contains(s))
	}

	pub fn validate_ip(&self, ip: &str) -> bool {
		is_ip(ip)
	}
}
