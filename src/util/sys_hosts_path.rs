#[cfg(any(feature = "_dev", target_os = "windows"))]
use std::env;
use std::path::PathBuf;

use anyhow::Result;

#[cfg(all(not(feature = "_dev"), target_os = "windows"))]
pub fn get_sys_hosts_path() -> Result<PathBuf> {
	let sys_drive = env::var("SYSTEMDRIVE")?;
	let path = PathBuf::from(format!(
		"{}\\Windows\\System32\\drivers\\etc\\hosts",
		sys_drive
	));

	Ok(path)
}

#[cfg(all(not(feature = "_dev"), target_os = "macos"))]
pub fn get_sys_hosts_path() -> Result<PathBuf> {
	Ok(PathBuf::from("/etc/hosts"))
}

#[cfg(feature = "_dev")]
pub fn get_sys_hosts_path() -> Result<PathBuf> {
	let mut path = env::current_dir()?;

	path.push("tmp");
	path.push("hosts");

	Ok(path)
}
