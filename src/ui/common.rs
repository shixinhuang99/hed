use egui::{Key, KeyboardShortcut, Modifiers, Ui};

pub fn set_button_padding(ui: &mut Ui) {
	let spacing = ui.spacing_mut();
	spacing.button_padding.x = 16.0;
	spacing.button_padding.y = 6.0;
}

pub fn save_btn_shortcut() -> KeyboardShortcut {
	KeyboardShortcut::new(Modifiers::CTRL, Key::S)
}

pub fn reset_btn_shortcut() -> KeyboardShortcut {
	KeyboardShortcut::new(Modifiers::CTRL, Key::R)
}

pub fn format_btn_shortcut() -> KeyboardShortcut {
	KeyboardShortcut::new(Modifiers::SHIFT | Modifiers::ALT, Key::F)
}
