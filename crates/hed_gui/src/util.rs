use std::{env, path::PathBuf};

use anyhow::Result;

#[cfg(not(feature = "_dev"))]
pub fn get_sys_hosts_path() -> Result<PathBuf> {
	let sys_drive = env::var("SYSTEMDRIVE")?;

	let mut path = PathBuf::from(sys_drive);

	path.push("Windows");
	path.push("System32");
	path.push("drivers");
	path.push("etc");
	path.push("hosts");

	Ok(path)
}

#[cfg(feature = "_dev")]
pub fn get_sys_hosts_path() -> Result<PathBuf> {
	let mut path = env::current_dir().unwrap();

	path.push("tmp");
	path.push("hosts");

	Ok(path)
}
