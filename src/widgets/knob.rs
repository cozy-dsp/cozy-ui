use colorgrad::{BasisGradient, Color, Gradient, GradientBuilder};
use egui::{
    epaint::PathShape, pos2, remap_clamp, Color32, Painter, Response, Sense, Stroke, TextStyle, Ui,
    Vec2, Widget, WidgetText,
};
use once_cell::sync::Lazy;

use crate::{
    colors::{HIGHLIGHT, PURPLE_COL32, WIDGET_BACKGROUND_COL32},
    util::{generate_arc, get_set::Operation},
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

pub fn knob<GetSet: FnMut(Operation<f32>) -> f32, Start: Fn(), End: Fn()>(
    id: &str,
    diameter: f32,
    value: GetSet,
    begin_set: Start,
    end_set: End,
) -> Knob<'_, GetSet, Start, End> {
    Knob::new(id, diameter, value, begin_set, end_set)
}

pub struct Knob<'a, GetSet: FnMut(Operation<f32>) -> f32, Start: Fn(), End: Fn()> {
    id: &'a str,
    label: Option<WidgetText>,
    description: Option<WidgetText>,
    diameter: f32,
    value: GetSet,
    begin_set: Start,
    end_set: End,
    default: Option<f32>,
    modulated: Option<f32>,
}

impl<'a, GetSet: FnMut(Operation<f32>) -> f32, Start: Fn(), End: Fn()> Knob<'a, GetSet, Start, End> {
    pub const fn new(
        id: &'a str,
        diameter: f32,
        value: GetSet,
        begin_set: Start,
        end_set: End,
    ) -> Self {
        Self {
            id,
            diameter,
            value,
            begin_set,
            end_set,
            label: None,
            description: None,
            default: None,
            modulated: None,
        }
    }

    pub fn label(mut self, label: impl Into<WidgetText>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description(mut self, description: impl Into<WidgetText>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub const fn default_value(mut self, default: f32) -> Self {
        self.default = Some(default);
        self
    }

    pub const fn modulated_value(mut self, value: f32) -> Self {
        self.modulated = Some(value);
        self
    }
}

impl<'a, GetSet: FnMut(Operation<f32>) -> f32, Start: Fn(), End: Fn()> Widget
    for Knob<'a, GetSet, Start, End>
{
    fn ui(mut self, ui: &mut Ui) -> Response {
        let mut desired_size = Vec2::splat(self.diameter + 5.0);
        let galley = self.label.map_or_else(
            || None,
            |label| {
                let galley = label.into_galley(ui, Some(egui::TextWrapMode::Extend), desired_size.x, TextStyle::Body);
                let height_difference = galley.size().y + ui.spacing().item_spacing.y;
                desired_size.y += height_difference;
                desired_size.x = desired_size.x.max(galley.size().x);
                Some(galley)
            },
        );
        let (full_rect, mut response) =
            ui.allocate_exact_size(desired_size, Sense::click_and_drag());
        if let Some(description) = self.description {
            response = response.on_hover_text_at_pointer(description);
        }
        let (rect, text_rect) = if galley.is_some() {
            let (rect, text_rect) =
                full_rect.split_top_bottom_at_y(full_rect.top() + (self.diameter + 5.0));
            (rect, Some(text_rect))
        } else {
            (full_rect, None)
        };
        let mut granular = false;
        let hovered = response.hovered() || response.dragged();

        if let Some(default) = self.default {
            if response.double_clicked() {
                (self.begin_set)();
                set(&mut self.value, default);
                (self.end_set)();
            }
        }

        if response.hovered() {
            granular = response.ctx.input(|i| i.modifiers.shift);
        }

        if response.drag_started() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::None);
            (self.begin_set)();
        }

        if response.dragged() {
            let drag_delta = response.drag_delta();
            granular = response.ctx.input(|i| i.modifiers.shift);
            let diameter_scale = if granular { 4.0 } else { 2.0 };

            let delta = -(drag_delta.x + drag_delta.y);
            let mut new_value = get(&mut self.value);
            new_value += delta / (self.diameter * diameter_scale);
            new_value = new_value.clamp(0.0, 1.0);
            set(&mut self.value, new_value);

            response.mark_changed();
        } else if response.hovered()
            && response
                .ctx
                .input(|input| input.raw_scroll_delta.length() > 0.0)
        {
            (self.begin_set)();
            let drag_delta = response.ctx.input(|input| input.smooth_scroll_delta);
            granular = response.ctx.input(|i| i.modifiers.shift);
            let diameter_scale = if granular { 8.0 } else { 4.0 };

            let delta = -(drag_delta.x + drag_delta.y);
            let mut new_value = get(&mut self.value);
            new_value += delta / (self.diameter * diameter_scale);
            new_value = new_value.clamp(0.0, 1.0);
            set(&mut self.value, new_value);

            response.mark_changed();
            (self.end_set)();
        }

        if response.drag_stopped() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Default);
            (self.end_set)();
        }

        if ui.is_rect_visible(full_rect) {
            let value = get(&mut self.value);

            let radius = (self.diameter * 0.75) / 2.0;
            let background_radius = self.diameter / 2.0;
            let focus_ring_radius = (self.diameter * 0.90) / 2.0;

            let painter = Painter::new(ui.ctx().clone(), ui.layer_id(), rect);
            let id = self.id;

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

            generate_arc(
                &painter,
                painter.clip_rect().center(),
                radius,
                225.0_f32.to_radians(),
                -45.0_f32.to_radians(),
                Stroke::new(radius * 0.1, stroke_color),
            );

            let value_angle = remap_clamp(value, 0.0..=1.0, START_DEG..=END_DEG);

            star(&painter, value_angle, self.diameter);

            if let Some(modulated) = self.modulated {
                let modulated_angle = remap_clamp(modulated, 0.0..=1.0, START_DEG..=END_DEG);

                generate_arc(
                    &painter,
                    painter.clip_rect().center(),
                    radius * 0.75,
                    value_angle.to_radians(),
                    modulated_angle.to_radians(),
                    Stroke::new(radius * 0.1, Color32::from_rgb(133, 19, 173)),
                );
            }

            painter.circle_stroke(
                painter.clip_rect().center(),
                focus_ring_radius,
                Stroke::new(
                    focus_ring_radius * 0.07,
                    PURPLE_COL32.gamma_multiply(animated_hover),
                ),
            );

            let (tick_sin, tick_cos) = value_angle.to_radians().sin_cos();
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
