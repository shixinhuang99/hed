#[derive(Default)]
pub struct ProfileForm {
	pub name: String,
	pub error: String,
}

impl ProfileForm {
	pub fn reset(&mut self) {
		self.name.clear();
		self.error.clear();
	}

	pub fn validate(
		&mut self,
		check_exists: impl FnOnce(&str) -> bool,
	) -> bool {
		self.name = self.name.trim().to_string();
		if self.name.is_empty() {
			self.error = "name is empty".to_string();
			return false;
		}
		if check_exists(&self.name) {
			self.error = format!("`{}` already exists", self.name);
			return false;
		}
		true
	}
}
