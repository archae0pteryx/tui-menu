use crossterm::event::KeyCode;

use crate::app::App;

pub fn handle_key_event(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Esc => {
            app.should_quit = true;
        }
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        _ => {}
    }
}
