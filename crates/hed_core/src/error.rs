use std::{env, io, result};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HedError {
	#[error("failed to get SYSTEMDRIVE environment variable")]
	SystemDriveEnv(#[from] env::VarError),
	#[error("failed to parse hosts file")]
	Parse(#[from] io::Error),
}

pub type Result<T> = result::Result<T, HedError>;
