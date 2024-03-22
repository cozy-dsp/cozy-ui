use egui::{lerp, pos2, remap_clamp, vec2, Color32, Rect, Response, Rounding, Sense, Ui, Vec2, WidgetText};

use crate::colors::HIGHLIGHT_COL32;

use super::{get, set};

#[allow(clippy::too_many_arguments)]
pub fn slider<GetSet, Start, End, Text>(
    ui: &mut Ui,
    id: &str,
    description: Option<Text>,
    width: Option<f32>,
    mut value: GetSet,
    drag_started: Start,
    drag_ended: End,
    default: f32,
) -> Response
where
    GetSet: FnMut(Option<f32>) -> f32,
    Start: Fn(),
    End: Fn(),
    Text: Into<WidgetText>
{
    let desired_size = vec2(width.unwrap_or(ui.spacing().slider_width), 15.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());
    if let Some(description) = description {
        response = response.on_hover_text_at_pointer(description.into());
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
            drag_started();
        }
        let pointer_position = pointer_position.x;
        let normalized = remap_clamp(pointer_position, position_range, 0.0..=1.0);
        set(&mut value, normalized);
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
        drag_ended();
    }

    if response.double_clicked() {
        drag_started();
        set(&mut value, default);
        response.mark_changed();
        drag_ended();
    }

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect.shrink(5.0), Rounding::same(1.5), visuals.weak_bg_fill);

        let position_1d = lerp(position_range, get(&mut value));
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
