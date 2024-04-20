use std::f32::consts::PI;

use egui::{epaint::CubicBezierShape, Color32, Painter, Pos2, Shape, Stroke, Vec2};

pub fn generate_arc(painter: &Painter, center: Pos2, radius: f32, start_angle: f32, end_angle: f32, stroke: impl Into<Stroke>) {
    let mut result = Vec::new();
    recurse_arcs(&mut result, center, radius, start_angle, end_angle, stroke.into());
    painter.extend(result);
}

fn recurse_arcs(list: &mut Vec<Shape>, center: Pos2, radius: f32, start_angle: f32, end_angle: f32, stroke: Stroke) {
    let diff = end_angle - start_angle;
    let (start_angle, end_angle) = if diff.abs() < PI / 2.0 {
        (start_angle, end_angle)
    } else {
        let new_start_angle = start_angle + ((PI / 2.0) * diff.signum());
        recurse_arcs(list, center, radius, new_start_angle, end_angle, stroke);
        (start_angle, new_start_angle)
    };

    // Center of the circle
    let xc = center.x;
    let yc = center.y;

    // First control point
    let p1 = center + radius * Vec2::new(start_angle.cos(), -start_angle.sin());

    // Last control point
    let p4 = center + radius * Vec2::new(end_angle.cos(), -end_angle.sin());

    let a = p1 - center;
    let b = p4 - center;
    let q1 = a.length_sq();
    let q2 = q1 + a.dot(b);
    let k2 = (4.0 / 3.0) * ((2.0 * q1 * q2).sqrt() - q2) / (a.x * b.y - a.y * b.x);

    let p2 = Pos2::new(xc + a.x - k2 * a.y, yc + a.y + k2 * a.x);
    let p3 = Pos2::new(xc + b.x + k2 * b.y, yc + b.y - k2 * b.x);

    list.push(Shape::CubicBezier(CubicBezierShape::from_points_stroke([p1, p2, p3, p4], false, Color32::TRANSPARENT, stroke)));
}
