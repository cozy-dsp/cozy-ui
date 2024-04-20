use colors::{BACKGROUND, WIDGET_BACKGROUND_COL32};
use egui::{epaint::Shadow, vec2, Color32, Context, InnerResponse, Ui};

pub mod util;

pub mod widgets;

pub mod colors;

pub fn setup(ctx: &Context) {
    ctx.style_mut(|style| {
        style.visuals.popup_shadow = Shadow {
            color: Color32::BLACK,
            offset: vec2(1.0, 2.0),
            blur: 5.0,
            spread: 1.0,
        };
        style.visuals.panel_fill = Color32::from_rgb(BACKGROUND.0, BACKGROUND.1, BACKGROUND.2);
        style.visuals.widgets.inactive.weak_bg_fill = WIDGET_BACKGROUND_COL32;
        style.visuals.widgets.hovered.weak_bg_fill = WIDGET_BACKGROUND_COL32;
        style.visuals.widgets.active.weak_bg_fill = WIDGET_BACKGROUND_COL32;
        style.visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
    });
}

pub fn centered<R>(
    ctx: &Context,
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let layer_id = egui::LayerId::new(egui::Order::Foreground, ui.next_auto_id());
    ui.with_layer_id(layer_id, |ui| {
        let resp = add_contents(ui);

        let shift_x = ui.available_width() / 2.0;
        ctx.set_transform_layer(
            layer_id,
            egui::emath::TSTransform::from_translation(egui::vec2(shift_x, 0.0)),
        );
        resp
    })
}
