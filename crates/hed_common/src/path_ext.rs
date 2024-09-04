use std::{
	ffi::OsStr,
	path::{Component, Path, PathBuf},
};

pub trait PathExt {
	fn join_as_components<T>(self, p: &T) -> PathBuf
	where
		T: AsRef<OsStr> + ?Sized;
}

impl PathExt for PathBuf {
	fn join_as_components<T>(mut self, p: &T) -> PathBuf
	where
		T: AsRef<OsStr> + ?Sized,
	{
		for comp in Path::new(p)
			.components()
			.filter(|comp| matches!(comp, Component::Normal(_)))
		{
			self.push(comp);
		}

		self
	}
}
