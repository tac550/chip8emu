use std::{time::Duration, fs::File, io::{self, Read}};

use chip8exe::{Chip8State, chip8_tick, chip8_reset};
use ratatui::widgets::{ListState, TableState};

//                           0.5 Hz         1 Hz           5 Hz         10 Hz        100 Hz      1000 Hz    1 MHz
const DURATIONS: [u64; 7] = [2_000_000_000, 1_000_000_000, 200_000_000, 100_000_000, 10_000_000, 1_000_000, 1000];
//                           60 Hz
pub const TIMER_RATE: u128 = 16_666_666;

pub struct Failure {
    pub panic_message: String,
    pub last_instr_count: u64,
}

#[derive(Default)]
pub struct App {
    pub last_failure: Option<Failure>,
    pub should_quit: bool,
    tick_rate: Option<Duration>,
    selected_rate: usize,
    pub instr_count: u64,
    pub chip_state: Chip8State,

    pub stack_state: ListState,
    pub memory_state: TableState,
    pub mem_row_sel_override: Option<usize>,
}

impl App {
    pub fn new(last_failure: Option<Failure>) -> Self {
        Self {
            last_failure,
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
        self.tick_rate.unwrap_or(Duration::from_millis(16))
    }

    pub fn is_paused(&self) -> bool {
        self.tick_rate.is_none()
    }

    pub fn inc_tick_rate(&mut self) {
        if let Some(duration) = &mut self.tick_rate {
            if let Some(rate) = DURATIONS.get(self.selected_rate + 1) {
                self.selected_rate += 1;
                *duration = Duration::from_nanos(*rate);
            }
        } else {
            let sel_rate = DURATIONS.len() - 1;
            self.tick_rate = Some(Duration::from_nanos(DURATIONS[sel_rate])); // 1 MHz
            self.selected_rate = sel_rate;
        }
    }

    pub fn dec_tick_rate(&mut self) {
        if let Some(duration) = &mut self.tick_rate {
            let sub = self.selected_rate.saturating_sub(1);
            if let Some(rate) = DURATIONS.get(sub) {
                self.selected_rate = sub;
                *duration = Duration::from_nanos(*rate);
            }
        } else {
            self.tick_rate = Some(Duration::from_nanos(DURATIONS[0])); // 0.5 Hz
            self.selected_rate = 0;
        }
    }

    pub fn pause_tick(&mut self) {
        self.tick_rate = None;
    }

    pub fn on_tick(mut self, time_passed: u32) -> Self {
        chip8_tick(&mut self.chip_state, time_passed);
        self.instr_count = self.instr_count.saturating_add(1);

        if self.last_failure.is_some() {
            self.last_failure = None;
        }

        self
    }

    pub fn reset(&mut self) {
        chip8_reset(&mut self.chip_state);
        self.pause_tick();
        self.instr_count = 0;
        self.chip_state.input = 0;
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