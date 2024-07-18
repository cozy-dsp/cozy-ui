use egui::{
    lerp, pos2, remap_clamp, vec2, Color32, Rect, Rounding, Sense, Ui, Vec2, Widget, WidgetText,
};

use crate::{colors::HIGHLIGHT_COL32, util::get_set::Operation};

use super::{get, set};

pub const fn slider<GetSet: FnMut(Operation<f32>) -> f32, Start: Fn(), End: Fn()>(
    id: &str,
    value: GetSet,
    begin_set: Start,
    end_set: End,
) -> Slider<GetSet, Start, End> {
    Slider::new(id, value, begin_set, end_set)
}

#[must_use]
pub struct Slider<'a, GetSet: FnMut(Operation<f32>) -> f32, Start: Fn(), End: Fn()> {
    id: &'a str,
    description: Option<WidgetText>,
    width: Option<f32>,
    default: Option<f32>,
    value: GetSet,
    begin_set: Start,
    end_set: End,
}

impl<'a, GetSet: FnMut(Operation<f32>) -> f32, Start: Fn(), End: Fn()>
    Slider<'a, GetSet, Start, End>
{
    pub const fn new(id: &'a str, value: GetSet, begin_set: Start, end_set: End) -> Self {
        Self {
            id,
            description: None,
            width: None,
            default: None,
            value,
            begin_set,
            end_set,
        }
    }

    /// Sets the description (flavor text shown in a tooltip)
    pub fn description(mut self, description: impl Into<WidgetText>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets a fixed width for the slider. If this isn't set, the ``slider_width`` is used instead
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the value the slider will reset to when the user double clicks on the slider
    pub const fn default_value(mut self, default: f32) -> Self {
        self.default = Some(default);
        self
    }
}

impl<GetSet: FnMut(Operation<f32>) -> f32, Start: Fn(), End: Fn()> Widget
    for Slider<'_, GetSet, Start, End>
{
    fn ui(mut self, ui: &mut Ui) -> egui::Response {
        let id = self.id;

        let desired_size = vec2(self.width.unwrap_or(ui.spacing().slider_width), 15.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());
        if let Some(description) = self.description {
            response = response.on_hover_text_at_pointer(description);
        }

        let handle_radius = rect.height() / 2.5;
        let handle_radius_aspect = handle_radius * 0.5;
        let position_range = rect.x_range().shrink(handle_radius);

        if let Some(pointer_position) = response.interact_pointer_pos() {
            if ui.memory(|mem| {
                !mem.data
                    .get_temp(format!("slider_{id}_begin_set").into())
                    .unwrap_or(false)
            }) {
                ui.memory_mut(|mem| {
                    mem.data
                        .insert_temp(format!("slider_{id}_begin_set").into(), true);
                });
                (self.begin_set)();
            }
            let pointer_position = pointer_position.x;
            let normalized = remap_clamp(pointer_position, position_range, 0.0..=1.0);
            set(&mut self.value, normalized);
            response.mark_changed();
        }

        if ui.input(|input| input.pointer.primary_released())
            && ui.memory(|mem| {
                mem.data
                    .get_temp(format!("slider_{id}_begin_set").into())
                    .unwrap_or(false)
            })
        {
            ui.memory_mut(|mem| {
                mem.data
                    .insert_temp(format!("slider_{id}_begin_set").into(), false);
            });
            (self.end_set)();
        }

        if let Some(default) = self.default {
            if response.double_clicked() {
                (self.begin_set)();
                set(&mut self.value, default);
                response.mark_changed();
                (self.end_set)();
            }
        }

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect.shrink(5.0), Rounding::same(1.5), visuals.weak_bg_fill);

            let position_1d = lerp(position_range, get(&mut self.value));
            let center = pos2(position_1d, rect.center().y);

            let mut trailing_rect = rect.shrink(5.0);
            trailing_rect.max.x = center.x;
            painter.rect_filled(trailing_rect, Rounding::same(1.5), HIGHLIGHT_COL32);

            let v = vec2(handle_radius_aspect, handle_radius);
            let expansion = ui.ctx().animate_value_with_time(
                format!("slider_{id}_expansion").into(),
                ui.style().interact(&response).expansion,
                ui.style().animation_time,
            );
            let v = v + Vec2::splat(expansion);
            let handle_rect = Rect::from_center_size(center, 2.0 * v);
            painter.rect_filled(handle_rect, Rounding::same(2.0), Color32::WHITE);
        }

        response
    }
}
