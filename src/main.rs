use iced::widget::{
    center, column, combo_box, scrollable, text, vertical_space,
};
use iced::{Center, Element, Fill};

pub fn main() -> iced::Result {
    iced::run("Modulation measurer", TestApp::update, TestApp::view)
}


#[derive(Debug)]
struct TestApp {
    units: combo_box::State<ChartUnit>,
    selected_unit: Option<ChartUnit>,
    text: String,
}

impl TestApp {
    fn new() -> Self {
        todo!()
    }

    fn update(&mut self, message: TestApp) {
        todo!()
    }

    fn view(&self) -> Element<TestApp> {
        todo!()
    }
}

impl Default for TestApp {
    fn default() -> Self {
        TestApp::new()
    }
}


#[derive(Default, Debug, Clone)]
pub enum ChartUnit {
    #[default]
    Ns, 
    Us,
    Ms,
}




fn signal_frequency(range: f64, unit: ChartUnit) -> String {
    let period:f64 = match unit {
        ChartUnit::Ns => range * 1e-9,
        ChartUnit::Us => range * 1e-6,
        ChartUnit::Ms => range * 1e-3,
    };

    let period = 1.0 / period;

    let log10 = period.log10();
    let exp3 = (log10 / 3.0f64).floor() * 3.0f64; 
    let prefix = match exp3 {
            0.0 => "Hz",
            3.0 => "kHz",
            6.0 => "MHz",
            9.0 => "THz",
            _ => "Fail",
    };
    let factor = 10f64.powf(exp3);
    let value = period as f64 / factor as f64;
    // println!("log10: {} \n exp3: {} \n prefix: {} \n factor: {} \n value: {}", log10, exp3, prefix, factor, value);
    format!("{value:.3} {prefix}")
}