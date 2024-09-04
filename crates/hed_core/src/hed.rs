use std::path::PathBuf;

use indexmap::IndexMap;
use tokio::fs;

use crate::{error::Result, ip_hosts::IpHosts};

pub struct Hed {
	pub hosts_path: PathBuf,
	pub hosts_content: String,
	pub ip_hosts_map: IndexMap<String, IpHosts>,
}

impl Hed {
	pub fn new(hosts_path: PathBuf) -> Self {
		Self {
			hosts_path,
			hosts_content: String::new(),
			ip_hosts_map: IndexMap::new(),
		}
	}

	pub async fn parse(&mut self) -> Result<()> {
		let content = fs::read_to_string(&self.hosts_path).await?;

		self.hosts_content.clear();
		self.ip_hosts_map.clear();

		for raw_line in content.lines() {
			let line = raw_line.trim();
			if line.starts_with('#') {
				continue;
			}
			if let Ok(new_ih) = line.parse::<IpHosts>() {
				if let Some(ih) = self.ip_hosts_map.get_mut(&new_ih.ip) {
					ih.hosts.extend(new_ih.hosts);
				} else {
					self.ip_hosts_map.insert(new_ih.ip.clone(), new_ih);
				}
			}
		}

		self.hosts_content = content;

		Ok(())
	}
}
