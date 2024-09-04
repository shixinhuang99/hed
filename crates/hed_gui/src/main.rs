mod app;
mod util;

use anyhow::Result;
use eframe::{egui::ViewportBuilder, run_native, NativeOptions};

use app::App;

fn main() {
	if let Err(err) = run() {
		eprintln!("{}", err);
	}
}

fn run() -> Result<()> {
	let options = NativeOptions {
		viewport: ViewportBuilder::default().with_inner_size([800.0, 600.0]),
		..NativeOptions::default()
	};

	run_native("Hed", options, Box::new(|_| Ok(Box::new(App::new()))))?;

	Ok(())
}
