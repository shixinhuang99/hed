#[cfg(any(feature = "_dev", target_os = "windows"))]
use std::env;
use std::path::PathBuf;

use anyhow::Result;
#[cfg(feature = "_dev")]
use hed_common::PathExt;

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
	let path =
		env::current_dir()?.join_as_components("crates/hed_gui/tmp/hosts");

	Ok(path)
}
