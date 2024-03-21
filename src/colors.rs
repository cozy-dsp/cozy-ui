use colorgrad::Color;
use egui::Color32;

pub const BACKGROUND: (u8, u8, u8) = (69, 65, 73);
pub const WIDGET_BACKGROUND: (u8, u8, u8) = (75, 54, 78);
pub const HIGHLIGHT: (u8, u8, u8) = (255, 45, 128);
pub const PURPLE: (u8, u8, u8) = (118, 72, 151);

pub const PURPLE_COL32: Color32 = Color32::from_rgb(118, 72, 151);
pub const WIDGET_BACKGROUND_COL32: Color32 = Color32::from_rgb(
    WIDGET_BACKGROUND.0,
    WIDGET_BACKGROUND.1,
    WIDGET_BACKGROUND.2,
);
