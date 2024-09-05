use std::{io, result};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HedError {
	#[error("failed to parse hosts file")]
	Parse(#[from] io::Error),
}

pub type Result<T> = result::Result<T, HedError>;
