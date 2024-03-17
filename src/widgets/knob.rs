use colorgrad::{BasisGradient, Color, Gradient, GradientBuilder};
use egui::{epaint::PathShape, remap_clamp, Color32, Painter, Response, Sense, Stroke, Ui, Vec2};
use once_cell::sync::Lazy;

use crate::util::CIRCLE_POINTS;

use super::{get, set};

const LOWER_DEG: usize = 45;
const HIGHER_DEG: usize = 315;

pub fn knob<GetSet, Start, End>(
    ui: &mut Ui,
    id: &str,
    diameter: f32,
    mut value: GetSet,
    drag_started: Start,
    drag_ended: End,
    default: f32,
) -> Response
where
    GetSet: FnMut(Option<f32>) -> f32,
    Start: Fn(),
    End: Fn(),
{
    static TRACK_GRADIENT: Lazy<BasisGradient> = Lazy::new(|| {
        GradientBuilder::new()
            .colors(&[
                Color::from_html("#ff0000").unwrap(),
                Color::from_html("#de07db").unwrap(),
            ])
            .mode(colorgrad::BlendMode::Oklab)
            .build()
            .unwrap()
    });
    let desired_size = Vec2::splat(diameter + 5.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());
    let mut granular = false;

    if response.double_clicked() {
        drag_started();
        set(&mut value, default);
        drag_ended();
    }

    if response.contains_pointer() && response.ctx.input(|i| i.pointer.primary_down()) {
        granular = response.ctx.input(|i| i.modifiers.shift);
    }

    if response.drag_started() {
        drag_started();
    }

    if response.dragged() {
        let drag_delta = response.drag_delta();
        granular = response.ctx.input(|i| i.modifiers.shift);
        let diameter_scale = if granular { 4.0 } else { 2.0 };

        let delta = -(drag_delta.x + drag_delta.y);
        let mut new_value = get(&mut value);
        new_value += delta / (diameter * diameter_scale);
        new_value = new_value.clamp(0.0, 1.0);
        set(&mut value, new_value);

        response.mark_changed();
    }

    if response.drag_released() {
        drag_ended();
    }

    if ui.is_rect_visible(rect) {
        let value = get(&mut value);

        let radius = diameter / 2.0;

        let painter = Painter::new(ui.ctx().clone(), ui.layer_id(), rect);

        let animated_granular = ui
            .ctx()
            .animate_bool(format!("knob_{id}_granular").into(), granular);
        let color = TRACK_GRADIENT.at(animated_granular).to_rgba8();
        let stroke_color = Color32::from_rgb(color[0], color[1], color[2]);

        knob_track(&painter, radius, stroke_color);

        #[allow(
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation,
            clippy::cast_precision_loss
        )]
        let tick_angle =
            remap_clamp(value, 0.0..=1.0, HIGHER_DEG as f32..=LOWER_DEG as f32).round() as usize;
        let (tick_sin, tick_cos) = CIRCLE_POINTS[tick_angle - 1];
        painter.line_segment(
            [
                painter.clip_rect().center()
                    + Vec2::new(radius * 0.5 * tick_sin, radius * 0.5 * tick_cos),
                painter.clip_rect().center() + Vec2::new(radius * tick_sin, radius * tick_cos),
            ],
            Stroke::new(2.0, Color32::WHITE),
        );
    }

    response
}

fn knob_track(painter: &Painter, radius: f32, stroke_color: Color32) {
    let mut points = Vec::with_capacity(HIGHER_DEG - LOWER_DEG + 1);
    for deg in LOWER_DEG..=HIGHER_DEG {
        let (sin, cos) = CIRCLE_POINTS[deg - 1];

        points.push(painter.clip_rect().center() + Vec2::new(radius * sin, radius * cos));
    }

    painter.add(PathShape::line(points, Stroke::new(1.5, stroke_color)));
}
