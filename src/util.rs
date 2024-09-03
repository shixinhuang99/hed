use std::{env, path::PathBuf};

use anyhow::Result;

pub fn get_sys_hosts_path() -> Result<PathBuf> {
	let sys_drive = env::var("SYSTEMDRIVE")?;

	let mut path = PathBuf::from(sys_drive);

	path.push("Windows/System32/drivers/etc/hosts");

	Ok(path)
}
