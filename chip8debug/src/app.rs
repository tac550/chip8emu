use std::time::Duration;

use chip8exe::Chip8State;

#[derive(Default)]
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    tick_rate: Option<Duration>,
    pub chip_state: Chip8State,
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
}