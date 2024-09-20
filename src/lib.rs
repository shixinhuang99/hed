pub mod consts;
mod core;
mod ui;
mod util;

pub use crate::core::Hed;
use ui::{editor, editor_header, header, left_side};

impl eframe::App for Hed {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		header(ctx, self);
		left_side(ctx, self);
		editor_header(ctx, self);
		editor(ctx, self);
		self.handle_event();
	}
}
