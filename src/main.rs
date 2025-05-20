use eframe::{egui};


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Default, Debug, Clone, Copy)]
pub enum LengthUnit {
    #[default]
    Ns,
    Us,
    Ms,
}

#[derive(Default)]
struct MyEguiApp {
    graphxsize: String,
    unit: LengthUnit,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.horizontal(|ui| {
                ui.label("Graph Size horizontal");
                ui.text_edit_singleline(&mut self.graphxsize);
            });
        });
    }
}
