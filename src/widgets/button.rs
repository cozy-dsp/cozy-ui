use colorgrad::{BasisGradient, Color, Gradient, GradientBuilder};
use egui::{
    pos2, vec2, Color32, NumExt, Rect, Rounding, Sense, Stroke, TextStyle, Ui, Vec2, Widget,
    WidgetInfo, WidgetText, WidgetType,
};
use once_cell::sync::Lazy;

use crate::colors::{BACKGROUND, HIGHLIGHT};

use super::{get, set};

static LIGHT_GRADIENT: Lazy<BasisGradient> = Lazy::new(|| {
    GradientBuilder::new()
        .colors(&[Color::from(BACKGROUND), Color::from(HIGHLIGHT)])
        .mode(colorgrad::BlendMode::Oklab)
        .build()
        .unwrap()
});

pub fn toggle<GetSet: FnMut(Option<bool>) -> bool, Start: Fn(), End: Fn()>(
    id: &str,
    text: impl Into<WidgetText>,
    value: GetSet,
    begin_set: Start,
    end_set: End,
) -> ButtonToggle<GetSet, Start, End> {
    ButtonToggle::new(id, text.into(), value, begin_set, end_set)
}

pub struct ButtonToggle<'a, GetSet: FnMut(Option<bool>) -> bool, Start: Fn(), End: Fn()> {
    id: &'a str,
    description: Option<WidgetText>,
    value: GetSet,
    small: bool,
    text: WidgetText,
    begin_set: Start,
    end_set: End,
}

impl<'a, GetSet: FnMut(Option<bool>) -> bool, Start: Fn(), End: Fn()>
    ButtonToggle<'a, GetSet, Start, End>
{
    pub fn new(
        id: &'a str,
        text: impl Into<WidgetText>,
        value: GetSet,
        begin_set: Start,
        end_set: End,
    ) -> Self {
        Self {
            id,
            description: None,
            value,
            small: false,
            text: text.into(),
            begin_set,
            end_set,
        }
    }

    pub fn description(mut self, description: impl Into<WidgetText>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn small(mut self) -> Self {
        self.small = true;
        self
    }
}

impl<GetSet: FnMut(Option<bool>) -> bool, Start: Fn(), End: Fn()> Widget
    for ButtonToggle<'_, GetSet, Start, End>
{
    fn ui(mut self, ui: &mut Ui) -> egui::Response {
        let id = self.id;
        let mut button_padding = ui.spacing().button_padding;
        if self.small {
            button_padding.y = 0.0;
        }

        let text_wrap_width = 2.0f32.mul_add(-button_padding.x, ui.available_width());

        let galley = self
            .text
            .into_galley(ui, None, text_wrap_width, TextStyle::Button);

        let mut desired_size = Vec2::ZERO;
        desired_size.x += galley.size().x + 10.0;
        desired_size.y = desired_size.y.max(galley.size().y);
        desired_size += 2.0 * button_padding;
        if !self.small {
            desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
        }

        let (rect, mut response) = ui.allocate_at_least(desired_size, Sense::click());
        if let Some(description) = self.description {
            response = response.on_hover_text_at_pointer(description);
        }

        let mut new_value = get(&mut self.value);
        if response.clicked() {
            (self.begin_set)();
            new_value = !new_value;
            set(&mut self.value, new_value);
            (self.end_set)();
            response.mark_changed();
        }
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, galley.text()));

        let animated_value = ui
            .ctx()
            .animate_bool(format!("button_{id}_light").into(), new_value);

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            let color = LIGHT_GRADIENT.at(animated_value).to_rgba8();
            let light_color = Color32::from_rgb(color[0], color[1], color[2]);

            let (frame_expansion, frame_rounding, frame_fill, frame_stroke) = {
                let expansion = Vec2::splat(visuals.expansion);
                (
                    expansion,
                    visuals.rounding,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                )
            };
            ui.painter().rect(
                rect.expand2(frame_expansion),
                frame_rounding,
                frame_fill,
                frame_stroke,
            );

            let cursor_x = rect.min.x + button_padding.x + 10.0;

            let text_pos = pos2(cursor_x, 0.5f32.mul_add(-galley.size().y, rect.center().y));
            let light_rect_pos = pos2(rect.min.x + button_padding.x + 3.0, rect.center().y);
            let light_rect = Rect::from_center_size(light_rect_pos, vec2(4.0, rect.height() - 4.0));
            ui.painter()
                .rect(light_rect, Rounding::same(10.0), light_color, Stroke::NONE);
            ui.painter().galley(text_pos, galley, visuals.text_color());
        }

        if let Some(cursor) = ui.visuals().interact_cursor {
            if response.hovered {
                ui.ctx().set_cursor_icon(cursor);
            }
        }

        response
    }
}
