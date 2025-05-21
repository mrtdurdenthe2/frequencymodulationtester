use std::{fmt::format, ptr::null};

use eframe::{egui};


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
    println!("{}", signal_frequency(385400))
}

fn signal_frequency(period: u64)-> String {
    println!("{}", period);
    let hz:u64 = 1u64 / period;
    if hz == 0 {
        return "0 Hz, error".to_string();
    }
    let log10 = hz.checked_ilog10().unwrap_or_default();
    let exp3 = (log10 / 3) * 3;
    let prefix = match exp3 {
        0 => "Hz",
        3 => "kHz",
        6 => "MHz",
        9 => "THz",
        _ => "Hz",
    };
    let factor = 10u64.pow(exp3);
    let value = hz as f64 / factor as f64;
    format!("{value:.3} {prefix}")
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
    graphxsize: u64,
    unit: LengthUnit,
    posstart: u64,
    posend: u64,
    range: u64,
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
                ui.add(egui::DragValue::new(&mut self.posstart).range(0u64..=30000));
                if ui.button("Print").clicked() {
                    println!("{}", signal_frequency(self.posstart))
                }
            });
        });
    }
}
