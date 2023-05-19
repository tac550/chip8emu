use std::{time::Duration, fs::File, io::{self, Read}};

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

    pub fn load_program(&mut self, path: &str) -> io::Result<()> {
        let mut f = File::open(path)?;
        let mut bytes_read = 0;
        loop {
            let read = f.read(&mut self.chip_state.memory[0x200 + bytes_read..])?;
            if read == 0 {
                break;
            }
            bytes_read += read;
        }
        Ok(())
    }

    pub fn get_tick_rate(&self) -> Duration {
        self.tick_rate.unwrap_or(Duration::MAX)
    }

    pub fn on_tick(&mut self) {

    }

    pub fn disp_frequency(&self) -> String {
        let mut display = String::from("Frequency: ");
        if let Some(rate) = self.tick_rate {
            #[allow(clippy::cast_precision_loss)]
            let hz = (1.0 / rate.as_nanos() as f64) * 1_000_000_000.0;
            if hz >= 1_000_000.0 {
                display.push_str(&format!("{} MHz", hz / 1_000_000.0));
            } else {
                display.push_str(&format!("{hz} Hz"));
            }
        } else {
            display.push_str("Paused");
        }

        display
    }
}