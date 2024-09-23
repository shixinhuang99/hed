pub trait StringExt {
	fn make_trim(&mut self);

	fn to_split_whitespace_vec(&self) -> Vec<String>;
}

impl StringExt for String {
	fn make_trim(&mut self) {
		*self = self.trim().to_string();
	}

	fn to_split_whitespace_vec(&self) -> Vec<String> {
		self.split_whitespace().map(|s| s.to_string()).collect()
	}
}
