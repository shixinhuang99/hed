use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

#[macro_export]
macro_rules! static_global_id {
	($name: ident, $begin:literal) => {
		static $name: std::sync::LazyLock<$crate::util::GlobalID> =
			std::sync::LazyLock::new(|| $crate::util::GlobalID::new($begin));
	};
}

pub struct GlobalID {
	inner: AtomicUsize,
}

impl GlobalID {
	pub fn new(begin: usize) -> Self {
		Self {
			inner: AtomicUsize::new(begin),
		}
	}

	pub fn next(&self) -> usize {
		self.inner.fetch_add(1, Relaxed)
	}
}
