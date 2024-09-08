#[cfg(feature = "_dev")]
pub mod dev_window;
mod editor;
mod editor_header;
mod header;
mod left_side;
mod new_profile;
pub mod widgets;

pub use editor::editor;
pub use editor_header::editor_header;
pub use header::header;
pub use left_side::left_side;
