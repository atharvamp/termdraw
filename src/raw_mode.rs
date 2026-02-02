use crossterm::{cursor, event, execute, terminal};
use std::io::stdout;

pub fn init() {
    terminal::enable_raw_mode().unwrap();
    let _ = execute!(
        stdout(),
        cursor::Hide,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    );
}

pub fn restore() {
    let _ = execute!(
        stdout(),
        event::DisableMouseCapture,
        terminal::LeaveAlternateScreen,
        cursor::Show
    );
    terminal::disable_raw_mode().unwrap();
}
