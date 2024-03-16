use egui::{epaint::{CircleShape, PathShape}, remap_clamp, Color32, Painter, Pos2, Response, Sense, Stroke, Ui, Vec2};

use crate::util::CIRCLE_POINTS;

pub fn knob(ui: &mut Ui, diameter: f32, value: &mut f32) -> Response {
    let desired_size = Vec2::splat(diameter + 5.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    if response.dragged() {
        let drag_delta = response.drag_delta();

        let delta = -(drag_delta.x + drag_delta.y);
        *value += delta / (diameter * 2.0);
        *value = value.clamp(0.0, 1.0); 

        response.mark_changed()
    }

    if ui.is_rect_visible(rect) {
        let visuals = *ui.style().interact(&response);

        let mut points = Vec::new();
        let radius = diameter / 2.0;

        let painter = Painter::new(ui.ctx().clone(), ui.layer_id(), rect);

        for deg in 45..=315 {
            let (sin, cos) = CIRCLE_POINTS[deg - 1];

            points.push(painter.clip_rect().center() + Vec2::new(radius * sin, radius * cos));
        }

        painter.add(PathShape::line(points, Stroke::new(1.5, Color32::RED)));

        let tick_angle = remap_clamp(*value, 0.0..=1.0, 315.0..=45.0).round() as usize;
        let (tick_sin, tick_cos) = CIRCLE_POINTS[tick_angle - 1];
        painter.line_segment([painter.clip_rect().center() + Vec2::new(radius * 0.5 * tick_sin, radius * 0.5 * tick_cos), painter.clip_rect().center() + Vec2::new(radius * tick_sin, radius * tick_cos)], Stroke::new(2.0, Color32::WHITE));
    }

    response
}