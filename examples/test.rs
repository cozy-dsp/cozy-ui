use std::time::Duration;

use cozy_ui::widgets::button::toggle;
use cozy_ui::widgets::knob::knob;

use egui::CentralPanel;

use egui::util::History;

pub struct FrameHistory {
    frame_times: History<f32>,
}

impl Default for FrameHistory {
    fn default() -> Self {
        let max_age: f32 = 1.0;
        let max_len = (max_age * 300.0).round() as usize;
        Self {
            frame_times: History::new(0..max_len, max_age),
        }
    }
}

impl FrameHistory {
    // Called first
    pub fn on_new_frame(&mut self, now: f64, previous_frame_time: Option<f32>) {
        let previous_frame_time = previous_frame_time.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            *latest = previous_frame_time; // rewrite history now that we know
        }
        self.frame_times.add(now, previous_frame_time); // projected
    }

    pub fn mean_frame_time(&self) -> f32 {
        self.frame_times.average().unwrap_or_default()
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.frame_times.mean_time_interval().unwrap_or_default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!(
            "Mean CPU usage: {:.2} ms / frame",
            1e3 * self.mean_frame_time()
        ))
        .on_hover_text(
            "Includes all app logic, egui layout, tessellation, and rendering.\n\
            Does not include waiting for vsync.",
        );
        egui::warn_if_debug_build(ui);

        if !cfg!(target_arch = "wasm32") {
            egui::CollapsingHeader::new("📊 CPU usage history")
                .default_open(false)
                .show(ui, |ui| {
                    self.graph(ui);
                });
        }
    }

    fn graph(&mut self, ui: &mut egui::Ui) -> egui::Response {
        use egui::*;

        ui.label("egui CPU usage history");

        let history = &self.frame_times;

        // TODO(emilk): we should not use `slider_width` as default graph width.
        let height = ui.spacing().slider_width;
        let size = vec2(ui.available_size_before_wrap().x, height);
        let (rect, response) = ui.allocate_at_least(size, Sense::hover());
        let style = ui.style().noninteractive();

        let graph_top_cpu_usage = 0.010;
        let graph_rect = Rect::from_x_y_ranges(history.max_age()..=0.0, graph_top_cpu_usage..=0.0);
        let to_screen = emath::RectTransform::from_to(graph_rect, rect);

        let mut shapes = Vec::with_capacity(3 + 2 * history.len());
        shapes.push(Shape::Rect(epaint::RectShape::new(
            rect,
            style.rounding,
            ui.visuals().extreme_bg_color,
            ui.style().noninteractive().bg_stroke,
        )));

        let rect = rect.shrink(4.0);
        let color = ui.visuals().text_color();
        let line_stroke = Stroke::new(1.0, color);

        if let Some(pointer_pos) = response.hover_pos() {
            let y = pointer_pos.y;
            shapes.push(Shape::line_segment(
                [pos2(rect.left(), y), pos2(rect.right(), y)],
                line_stroke,
            ));
            let cpu_usage = to_screen.inverse().transform_pos(pointer_pos).y;
            let text = format!("{:.1} ms", 1e3 * cpu_usage);
            shapes.push(ui.fonts(|f| {
                Shape::text(
                    f,
                    pos2(rect.left(), y),
                    egui::Align2::LEFT_BOTTOM,
                    text,
                    TextStyle::Monospace.resolve(ui.style()),
                    color,
                )
            }));
        }

        let circle_color = color;
        let radius = 2.0;
        let right_side_time = ui.input(|i| i.time); // Time at right side of screen

        for (time, cpu_usage) in history.iter() {
            let age = (right_side_time - time) as f32;
            let pos = to_screen.transform_pos_clamped(Pos2::new(age, cpu_usage));

            shapes.push(Shape::line_segment(
                [pos2(pos.x, rect.bottom()), pos],
                line_stroke,
            ));

            if cpu_usage < graph_top_cpu_usage {
                shapes.push(Shape::circle_filled(pos, radius, circle_color));
            }
        }

        ui.painter().extend(shapes);

        response
    }
}

const SAMPLES: usize = 1024;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            cozy_ui::setup(&cc.egui_ctx);
            Box::<TestApp>::default()
        }),
    )
}

struct TestApp {
    knob: f32,
    knob2: f32,
    button: bool,
    button2: bool,
    frame_history: FrameHistory,
    frame_idx: usize,
    frame_usages: [f32; SAMPLES],
}

impl Default for TestApp {
    fn default() -> Self {
        Self {
            knob: Default::default(),
            knob2: Default::default(),
            button: false,
            button2: false,
            frame_history: FrameHistory::default(),
            frame_idx: Default::default(),
            frame_usages: [0.0; SAMPLES],
        }
    }
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.frame_history
            .on_new_frame(ctx.input(|i| i.time), frame.info().cpu_usage);
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                knob(
                    ui,
                    "knob1",
                    50.0,
                    get_set(&mut self.knob),
                    || {},
                    || {},
                    0.5,
                );
                knob(
                    ui,
                    "knob2",
                    75.0,
                    get_set(&mut self.knob2),
                    || {},
                    || {},
                    0.5,
                );
                knob(
                    ui,
                    "knob3",
                    100.0,
                    get_set(&mut self.knob),
                    || {},
                    || {},
                    0.5,
                );
                knob(
                    ui,
                    "knob4",
                    125.0,
                    get_set(&mut self.knob2),
                    || {},
                    || {},
                    0.5,
                );
            });
            toggle(ui, "button1", get_set(&mut self.button), false, "button 1", || {}, || {});
            toggle(ui, "button2", get_set(&mut self.button2), false, "button 2", || {}, || {});
            ui.label(format!("fps: {}", self.frame_history.fps()));
            if let Some(usage) = frame.info().cpu_usage {
                self.frame_usages[self.frame_idx] = usage;
                self.frame_idx = (self.frame_idx + 1) % SAMPLES;
            }
            ui.label(format!(
                "frame time: {:#?}",
                Duration::from_secs_f32(self.frame_usages.iter().sum::<f32>() / SAMPLES as f32)
            ));
        });
        ctx.request_repaint();
    }
}

fn get_set<T>(value: &mut T) -> impl FnMut(Option<T>) -> T + '_
where
    T: Copy,
{
    |v| match v {
        Some(v) => {
            *value = v;
            v
        }
        None => *value,
    }
}
