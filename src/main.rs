use iced::{Element, Sandbox, Settings};
use iced::widget::{column, combo_box, text, text_input};

pub fn main() -> iced::Result {
    TestApp::run(Settings::default())
}

#[derive(Default)]
struct TestApp {
    unit_state: combo_box::State<ChartUnit>,
    selected_unit: ChartUnit,
    range_state: text_input::State<String>,
    range: String,
    output: String,
}

#[derive(Debug, Clone)]
enum Message {
    UnitSelected(ChartUnit),
    RangeChanged(String),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartUnit {
    #[default]
    Ns,
    Us,
    Ms,
}

impl ChartUnit {
    const ALL: [ChartUnit; 3] = [ChartUnit::Ns, ChartUnit::Us, ChartUnit::Ms];
}

impl std::fmt::Display for ChartUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartUnit::Ns => write!(f, "ns"),
            ChartUnit::Us => write!(f, "\u{03bc}s"),
            ChartUnit::Ms => write!(f, "ms"),
        }
    }
}

impl Sandbox for TestApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            unit_state: combo_box::State::new(ChartUnit::ALL.to_vec()),
            selected_unit: ChartUnit::default(),
            range_state: text_input::State::new(),
            range: String::from("0"),
            output: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Modulation measurer")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::UnitSelected(unit) => {
                self.selected_unit = unit;
            }
            Message::RangeChanged(value) => {
                self.range = value;
            }
        }
        self.update_output();
    }

    fn view(&mut self) -> Element<Message> {
        column![
            combo_box(&mut self.unit_state, &ChartUnit::ALL[..], Some(self.selected_unit), Message::UnitSelected),
            text_input("Range", &self.range, Message::RangeChanged),
            text(&self.output)
        ]
        .into()
    }
}

impl TestApp {
    fn update_output(&mut self) {
        if let Ok(value) = self.range.trim().parse::<f64>() {
            self.output = signal_frequency(value, self.selected_unit);
        } else {
            self.output.clear();
        }
    }
}

fn signal_frequency(range: f64, unit: ChartUnit) -> String {
    let period = match unit {
        ChartUnit::Ns => range * 1e-9,
        ChartUnit::Us => range * 1e-6,
        ChartUnit::Ms => range * 1e-3,
    };

    let freq = 1.0 / period;
    let log10 = freq.log10();
    let exp3 = (log10 / 3.0).floor() * 3.0;
    let prefix = match exp3 as i32 {
        0 => "Hz",
        3 => "kHz",
        6 => "MHz",
        9 => "GHz",
        12 => "THz",
        _ => "Hz",
    };
    let factor = 10f64.powf(exp3);
    let value = freq / factor;
    format!("{:.3} {}", value, prefix)
}

