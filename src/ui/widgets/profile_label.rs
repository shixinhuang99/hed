use egui::*;

pub struct ProfileLabel {
	selected: bool,
	enabled: bool,
	text: WidgetText,
	height: f32,
}

impl ProfileLabel {
	pub fn new(
		selected: bool,
		enabled: bool,
		text: impl Into<WidgetText>,
		height: f32,
	) -> Self {
		Self {
			selected,
			enabled,
			text: text.into(),
			height,
		}
	}
}

impl Widget for ProfileLabel {
	fn ui(self, ui: &mut Ui) -> Response {
		let Self {
			selected,
			enabled,
			text,
			height,
		} = self;

		ui.set_height(height);

		let dark_mode = ui.style().visuals.dark_mode;
		let button_padding = ui.spacing().button_padding;
		let total_extra = button_padding + button_padding;

		let wrap_width = ui.available_width() - total_extra.x;
		let galley = text.into_galley(
			ui,
			Some(TextWrapMode::Truncate),
			wrap_width,
			TextStyle::Button,
		);

		let (rect, response) = ui.allocate_at_least(
			vec2(ui.available_width(), ui.available_height()),
			Sense::click(),
		);

		if ui.is_rect_visible(response.rect) {
			let text_pos = ui
				.layout()
				.align_size_within_rect(
					galley.size(),
					rect.shrink2(button_padding),
				)
				.min;

			let visuals = ui.style().interact_selectable(&response, selected);

			if selected
				|| enabled || response.hovered()
				|| response.highlighted()
				|| response.has_focus()
			{
				ui.painter().rect(
					rect,
					visuals.rounding,
					if enabled {
						if dark_mode {
							Color32::DARK_GREEN
						} else {
							Color32::LIGHT_GREEN
						}
					} else {
						visuals.weak_bg_fill
					},
					visuals.bg_stroke,
				);
			}

			ui.painter().galley(text_pos, galley, visuals.text_color());
		}

		response
	}
}
