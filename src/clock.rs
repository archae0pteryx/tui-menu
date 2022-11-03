use std::time::{Duration, Instant};

use crossterm::event::KeyCode;

use crate::app::App;

pub trait Tickable {
    fn on_tick(&self);
}

pub struct EventClock {
    pub timeout: Duration,
    tick_rate: Duration,
    last_tick: Instant,
}

impl Default for EventClock {
    fn default() -> Self {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        Self {
            timeout: tick_rate,
            tick_rate,
            last_tick,
        }
    }
}

impl EventClock {
    pub fn new() -> Self {
        Self {
            ..EventClock::default()
        }
    }

    pub fn poll(&mut self) -> &mut Self {
        self.timeout = self
            .tick_rate
            .checked_sub(self.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        self
    }

    pub fn tick<T: Tickable>(&mut self, tickable: &mut T) {
        if self.last_tick.elapsed() >= self.tick_rate {
            tickable.on_tick();
            self.last_tick = Instant::now();
        }
    }

    pub fn handle_key_event() {
        
    }

}
