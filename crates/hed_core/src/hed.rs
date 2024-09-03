use std::{net::IpAddr, path::PathBuf};

use tokio::fs;

use crate::error::Result;

pub struct Hed {
	pub hosts_path: PathBuf,
	pub hosts_content: String,
	pub ip_host_list: Vec<IpHost>,
}

pub struct IpHost {
	pub ip: IpAddr,
	pub host_list: Vec<String>,
}

impl Hed {
	pub fn new(hosts_path: PathBuf) -> Self {
		Self {
			hosts_path,
			hosts_content: String::new(),
			ip_host_list: vec![],
		}
	}

	pub async fn parse(&mut self) -> Result<()> {
		let content = fs::read_to_string(&self.hosts_path).await?;

		self.hosts_content = content;

		Ok(())
	}
}
