#[derive(PartialEq, Eq, Default, Clone, Copy)]
pub enum ViewKind {
	All,
	#[default]
	Options,
	Text,
}

impl ViewKind {
	pub fn toogle(&mut self) {
		match self {
			ViewKind::Options => *self = ViewKind::Text,
			ViewKind::Text => *self = ViewKind::Options,
			_ => (),
		}
	}

	pub fn is_options(&self) -> bool {
		*self == ViewKind::Options
	}

	pub fn is_text(&self) -> bool {
		*self == ViewKind::Text
	}
}
