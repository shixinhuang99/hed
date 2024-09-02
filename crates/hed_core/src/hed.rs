use std::{env, path::PathBuf};

use crate::error::Result;

pub struct Hed {
	pub hosts_path: PathBuf,
}

pub struct HedOptions {
	hosts_path: Option<PathBuf>,
}

impl Hed {
	pub fn new(options: HedOptions) -> Result<Self> {
		let hosts_path = options.hosts_path.unwrap_or_else(|| {
			unimplemented!();
		});
		unimplemented!();
	}
}

fn get_sys_hosts_path() -> Result<PathBuf> {
	let sys_drive = env::var("SYSTEMDRIVE")?;

	let mut path = PathBuf::from(sys_drive);

	path.push("Windows/System32/drivers/etc/hosts");

	Ok(path)
}
