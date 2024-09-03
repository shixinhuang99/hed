use std::path::PathBuf;

use tokio::fs;

use crate::{error::Result, ip_hosts::IpHosts};

pub struct Hed {
	pub hosts_path: PathBuf,
	pub hosts_content: String,
	pub ip_hosts_list: Vec<IpHosts>,
}

impl Hed {
	pub fn new(hosts_path: PathBuf) -> Self {
		Self {
			hosts_path,
			hosts_content: String::new(),
			ip_hosts_list: vec![],
		}
	}

	pub async fn parse(&mut self) -> Result<()> {
		let content = fs::read_to_string(&self.hosts_path).await?;

		self.hosts_content.clear();
		self.ip_hosts_list.clear();

		for raw_line in content.lines() {
			let line = raw_line.trim();
			if line.starts_with('#') {
				continue;
			}
			if let Ok(ip_host) = line.parse::<IpHosts>() {
				self.ip_hosts_list.push(ip_host);
			}
		}

		self.hosts_content = content;

		Ok(())
	}
}
