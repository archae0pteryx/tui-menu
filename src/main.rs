#![allow(unused)]
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{Event, self, KeyCode};
use clock::EventClock;
use key_events::handle_key_event;
use tui::{backend::Backend, Terminal};

mod app;
mod clock;
mod term;
mod ui;
mod key_events;

use app::App;
use term::{create_term, quit_app};
use ui::ui;
mod menu;

fn main() -> Result<()> {
    let mut app = App::new()?;
    let mut terminal = create_term()?;
    run_app(&mut terminal, &mut app)?;
    quit_app(&mut terminal)?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let mut clock = EventClock::new();
    loop {
        clock.poll();

        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(clock.timeout)? {
            if let Event::Key(key) = event::read()? {
                handle_key_event(key.code, app);
            }
        }

        clock.tick(app);

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
