use std::time::Duration;

use chip8exe::Chip8State;
use tui::widgets::{ListState, TableState};

#[derive(Default)]
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    tick_rate: Option<Duration>,
    pub chip_state: Chip8State,

    pub stack_state: ListState,
    pub memory_state: TableState,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            ..Default::default()
        }
    }

    pub fn get_tick_rate(&self) -> Duration {
        self.tick_rate.unwrap_or(Duration::MAX)
    }

    pub fn on_tick(&mut self) {

    }

    pub fn disp_frequency(&self) -> String {
        let mut display = String::from("Frequency: ");
        if let Some(rate) = self.tick_rate {
            let hz = (1.0 / rate.as_nanos() as f64) * 1000000000.0;
            if hz >= 1000000.0 {
                display.push_str(&format!("{} MHz", hz / 1000000.0));
            } else {
                display.push_str(&format!("{} Hz", hz));
            }
        } else {
            display.push_str("Paused");
        }

        display
    }
}