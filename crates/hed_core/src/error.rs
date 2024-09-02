use std::env::VarError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HedError {
	#[error("failed to get SYSTEMDRIVE environment variable")]
	SystemDriveEnv(#[from] VarError),
}

pub type Result<T> = std::result::Result<T, HedError>;
