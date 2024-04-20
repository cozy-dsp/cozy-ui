use colorgrad::{BasisGradient, Color, Gradient, GradientBuilder};
use egui::{
    epaint::PathShape, pos2, remap_clamp, Color32, Painter, Response, Sense, Stroke, TextStyle, Ui,
    Vec2, WidgetText,
};
use once_cell::sync::Lazy;

use crate::{
    colors::{HIGHLIGHT, PURPLE_COL32, WIDGET_BACKGROUND_COL32},
    util::generate_arc,
};

use super::{get, set};

const START_DEG: f32 = 225.0;
const END_DEG: f32 = -45.0;

static TRACK_GRADIENT: Lazy<BasisGradient> = Lazy::new(|| {
    GradientBuilder::new()
        .colors(&[Color::from(HIGHLIGHT), Color::from_html("#de07db").unwrap()])
        .mode(colorgrad::BlendMode::Oklab)
        .build()
        .unwrap()
});

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn knob<GetSet, Start, End, Label, Description>(
    ui: &mut Ui,
    id: &str,
    label: Option<Label>,
    description: Option<Description>,
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
    Label: Into<WidgetText>,
    Description: Into<WidgetText>,
    WidgetText: From<Label>,
{
    let mut desired_size = Vec2::splat(diameter + 5.0);
    let galley = label.map_or_else(
        || None,
        |label| {
            let galley = WidgetText::from(label).into_galley(
                ui,
                Some(false),
                desired_size.x,
                TextStyle::Body,
            );
            let height_difference = galley.size().y + ui.spacing().item_spacing.y;
            desired_size.y += height_difference;
            desired_size.x = desired_size.x.max(galley.size().x);
            Some(galley)
        },
    );
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

    if response.hovered() {
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
            .input(|input| input.raw_scroll_delta.length() > 0.0)
    {
        drag_started();
        let drag_delta = response.ctx.input(|input| input.raw_scroll_delta);
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

    if response.drag_stopped() {
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

        generate_arc(&painter, painter.clip_rect().center(), radius, 225.0_f32.to_radians(), -45.0_f32.to_radians(), Stroke::new(radius * 0.1, stroke_color));

        painter.circle_stroke(
            painter.clip_rect().center(),
            focus_ring_radius,
            Stroke::new(
                focus_ring_radius * 0.07,
                PURPLE_COL32.gamma_multiply(animated_hover),
            ),
        );

        let tick_angle = remap_clamp(value, 0.0..=1.0, START_DEG..=END_DEG);

        star(&painter, tick_angle, diameter);

        let (tick_sin, tick_cos) = tick_angle.to_radians().sin_cos();
        let first_point = painter.clip_rect().center()
            + Vec2::new(radius * 0.5 * tick_cos, radius * 0.5 * -tick_sin);
        let second_point =
            painter.clip_rect().center() + Vec2::new(radius * tick_cos, radius * -tick_sin);
        painter.line_segment(
            [first_point, second_point],
            Stroke::new(background_radius * 0.15, Color32::WHITE),
        );
        painter.circle_filled(first_point, background_radius * 0.07, Color32::WHITE);
        painter.circle_filled(second_point, background_radius * 0.07, Color32::WHITE);

        if let Some(text_rect) = text_rect {
            if let Some(galley) = galley {
                ui.painter().galley(
                    pos2(
                        text_rect.center().x - galley.size().x / 2.0,
                        0.5f32.mul_add(-galley.size().y, text_rect.center().y),
                    ),
                    galley,
                    Color32::WHITE,
                );
            }
        }
    }

    response
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
