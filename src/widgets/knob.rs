use std::sync::Arc;

use colorgrad::{BasisGradient, Color, Gradient, GradientBuilder};
use egui::{epaint::PathShape, pos2, remap_clamp, Color32, Galley, Painter, Response, Rounding, Sense, Stroke, TextStyle, Ui, Vec2, WidgetText};
use once_cell::sync::Lazy;

use crate::{
    colors::{HIGHLIGHT, PURPLE_COL32, WIDGET_BACKGROUND_COL32},
    util::CIRCLE_POINTS,
};

use super::{get, set};

const LOWER_DEG: usize = 45;
const HIGHER_DEG: usize = 315;

static TRACK_GRADIENT: Lazy<BasisGradient> = Lazy::new(|| {
    GradientBuilder::new()
        .colors(&[Color::from(HIGHLIGHT), Color::from_html("#de07db").unwrap()])
        .mode(colorgrad::BlendMode::Oklab)
        .build()
        .unwrap()
});

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn knob<GetSet, Start, End, Text>(
    ui: &mut Ui,
    id: &str,
    label: Option<Text>,
    description: Option<Text>,
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
    Text: Into<WidgetText>,
    WidgetText: From<Text>
{
    let mut desired_size = Vec2::splat(diameter + 5.0);
    let galley = label.map_or_else(|| None, |label| {
            let galley = WidgetText::from(label).into_galley(ui, Some(false), desired_size.x, TextStyle::Body);
            let height_difference = galley.size().y + ui.spacing().item_spacing.y;
            desired_size.y += height_difference;
            desired_size.x = desired_size.x.max(galley.size().x);
            Some(galley)
        });
    let (full_rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());
    if let Some(description) = description {
        response = response.on_hover_text_at_pointer(description.into());
    }
    let (rect, text_rect) = if galley.is_some() {
        let (rect, text_rect) = full_rect.split_top_bottom_at_y(full_rect.top() + (diameter + 5.0));
        (rect, Some(text_rect))
    } else {
        (full_rect, None)
    };
    let mut granular = false;
    let hovered = response.hovered() || response.dragged();

    if response.double_clicked() {
        drag_started();
        set(&mut value, default);
        drag_ended();
    }

    if response.hovered() && response.ctx.input(|i| i.pointer.primary_down()) {
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
    } else if response.hovered()
        && response
            .ctx
            .input(|input| input.smooth_scroll_delta.length() > 0.0)
    {
        drag_started();
        let drag_delta = response.ctx.input(|input| input.smooth_scroll_delta);
        granular = response.ctx.input(|i| i.modifiers.shift);
        let diameter_scale = if granular { 4.0 } else { 2.0 };

        let delta = -(drag_delta.x + drag_delta.y);
        let mut new_value = get(&mut value);
        new_value += delta / (diameter * diameter_scale);
        new_value = new_value.clamp(0.0, 1.0);
        set(&mut value, new_value);

        response.mark_changed();
        drag_ended();
    }

    if response.drag_released() {
        drag_ended();
    }

    if ui.is_rect_visible(full_rect) {
        let value = get(&mut value);

        let radius = (diameter * 0.75) / 2.0;
        let background_radius = diameter / 2.0;
        let focus_ring_radius = (diameter * 0.90) / 2.0;

        let painter = Painter::new(ui.ctx().clone(), ui.layer_id(), rect);

        let animated_granular = ui
            .ctx()
            .animate_bool(format!("knob_{id}_granular").into(), granular);
        let animated_hover = ui
            .ctx()
            .animate_bool(format!("knob_{id}_hover").into(), hovered);
        let color = TRACK_GRADIENT.at(animated_granular).to_rgba8();
        let stroke_color = Color32::from_rgb(color[0], color[1], color[2]);

        painter.circle_filled(
            painter.clip_rect().center(),
            background_radius,
            WIDGET_BACKGROUND_COL32,
        );

        knob_track(&painter, radius, stroke_color);

        painter.circle_stroke(
            painter.clip_rect().center(),
            focus_ring_radius,
            Stroke::new(
                focus_ring_radius * 0.07,
                PURPLE_COL32.gamma_multiply(animated_hover),
            ),
        );

        #[allow(clippy::cast_precision_loss)]
        let tick_angle_f32 = remap_clamp(value, 0.0..=1.0, HIGHER_DEG as f32..=LOWER_DEG as f32);
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let tick_angle = tick_angle_f32.round() as usize;

        star(&painter, tick_angle_f32, diameter);

        let (tick_sin, tick_cos) = CIRCLE_POINTS[tick_angle - 1];
        let first_point = painter.clip_rect().center()
            + Vec2::new(radius * 0.5 * tick_sin, radius * 0.5 * tick_cos);
        let second_point =
            painter.clip_rect().center() + Vec2::new(radius * tick_sin, radius * tick_cos);
        painter.line_segment(
            [first_point, second_point],
            Stroke::new(background_radius * 0.15, Color32::WHITE),
        );
        painter.circle_filled(first_point, background_radius * 0.07, Color32::WHITE);
        painter.circle_filled(second_point, background_radius * 0.07, Color32::WHITE);

        if let Some(text_rect) = text_rect {
            if let Some(galley) = galley {
                ui.painter().galley(pos2(text_rect.center().x - galley.size().x / 2.0, 0.5f32.mul_add(-galley.size().y, text_rect.center().y)), galley, Color32::WHITE);
            }
        }
    }

    response
}

fn knob_track(painter: &Painter, radius: f32, stroke_color: Color32) {
    let mut points = Vec::with_capacity(HIGHER_DEG - LOWER_DEG + 1);
    for deg in LOWER_DEG..=HIGHER_DEG {
        let (sin, cos) = CIRCLE_POINTS[deg - 1];

        points.push(painter.clip_rect().center() + Vec2::new(radius * sin, radius * cos));
    }

    painter.add(PathShape::line(
        points,
        Stroke::new(radius * 0.1, stroke_color),
    ));
}

fn star(painter: &Painter, angle: f32, diameter: f32) {
    let angle = angle + 45.0;
    let (corner_1_sin, corner_1_cos) = angle.to_radians().sin_cos();
    let corner_1 = painter.clip_rect().center()
        + Vec2::new(
            (diameter * 0.2) * corner_1_sin,
            (diameter * 0.2) * corner_1_cos,
        );
    let (corner_2_sin, corner_2_cos) = (angle + 90.0).to_radians().sin_cos();
    let corner_2 = painter.clip_rect().center()
        + Vec2::new(
            (diameter * 0.2) * corner_2_sin,
            (diameter * 0.2) * corner_2_cos,
        );
    let (corner_3_sin, corner_3_cos) = (angle + 180.0).to_radians().sin_cos();
    let corner_3 = painter.clip_rect().center()
        + Vec2::new(
            (diameter * 0.2) * corner_3_sin,
            (diameter * 0.2) * corner_3_cos,
        );
    let (corner_4_sin, corner_4_cos) = (angle + 270.0).to_radians().sin_cos();
    let corner_4 = painter.clip_rect().center()
        + Vec2::new(
            (diameter * 0.2) * corner_4_sin,
            (diameter * 0.2) * corner_4_cos,
        );

    painter.add(PathShape::convex_polygon(
        vec![corner_1, corner_2, corner_3, corner_4],
        Color32::WHITE,
        Stroke::NONE,
    ));

    painter.circle_filled(corner_1, diameter * 0.15, WIDGET_BACKGROUND_COL32);
    painter.circle_filled(corner_2, diameter * 0.15, WIDGET_BACKGROUND_COL32);
    painter.circle_filled(corner_3, diameter * 0.15, WIDGET_BACKGROUND_COL32);
    painter.circle_filled(corner_4, diameter * 0.15, WIDGET_BACKGROUND_COL32);
}
