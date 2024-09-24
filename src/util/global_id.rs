use std::sync::{
	atomic::{AtomicUsize, Ordering::Relaxed},
	LazyLock,
};

pub static GLOBAL_ID: LazyLock<GlobalID> = LazyLock::new(GlobalID::new);

pub struct GlobalID {
	inner: AtomicUsize,
}

impl GlobalID {
	pub fn new() -> Self {
		Self {
			inner: AtomicUsize::new(1),
		}
	}

	pub fn next(&self) -> usize {
		self.inner.fetch_add(1, Relaxed)
	}
}
