#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use colors::BACKGROUND;
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
        style.visuals.panel_fill =
            Color32::from_rgb(BACKGROUND.0, BACKGROUND.1, BACKGROUND.2);
    });
}
