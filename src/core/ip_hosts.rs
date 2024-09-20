use std::collections::HashSet;

use crate::static_global_id;

static_global_id!(IP_HOST_ID, 1);
static_global_id!(HOST_ID, 1);

#[derive(Debug, Clone)]
pub struct IpHosts {
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

impl IpHosts {
	pub fn new(ip: &str, hosts: Vec<String>, enabled: bool) -> Self {
		let mut ip_hosts = Self {
			id: IP_HOST_ID.next(),
			ip: ip.to_string(),
			hosts: hosts
				.into_iter()
				.map(|name| Host {
					id: HOST_ID.next(),
					name,
					enabled,
				})
				.collect(),
		};

		ip_hosts.hosts_dedup();

		ip_hosts
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
				id: HOST_ID.next(),
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
}
