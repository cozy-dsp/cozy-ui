#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use colors::{BACKGROUND, WIDGET_BACKGROUND_COL32};
use egui::{epaint::Shadow, Color32, Context};

pub mod util;

pub mod widgets;

pub mod colors;

pub fn setup(ctx: &Context) {
    ctx.style_mut(|style| {
        style.visuals.popup_shadow = Shadow {
            extrusion: 1.5,
            color: Color32::BLACK,
        };
        style.visuals.panel_fill = Color32::from_rgb(BACKGROUND.0, BACKGROUND.1, BACKGROUND.2);
        style.visuals.widgets.inactive.weak_bg_fill = WIDGET_BACKGROUND_COL32;
        style.visuals.widgets.hovered.weak_bg_fill = WIDGET_BACKGROUND_COL32;
        style.visuals.widgets.active.weak_bg_fill = WIDGET_BACKGROUND_COL32;
        style.visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
    });
}
