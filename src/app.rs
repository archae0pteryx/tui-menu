use anyhow::Result;
use crossterm::{event::KeyCode, style::Color};
use tui::widgets::{BorderType, Borders, ListItem};

use crate::clock::Tickable;

pub struct App {
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        App { should_quit: false }
    }
}

impl Tickable for App {
    fn on_tick(&self) {
        //
    }
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self { ..App::default() })
    }

    pub fn on_tick(&self) {}

}
