use cozy_ui::widgets::knob::knob;
use egui::CentralPanel;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            Box::<TestApp>::default()
        }),
    )
}

#[derive(Default)]
struct TestApp {
    knob: f32
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            knob(ui, 50.0, &mut self.knob);
        });
    }
}