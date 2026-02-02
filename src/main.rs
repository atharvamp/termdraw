use crossterm::event::{self, Event};
use crossterm::{cursor, execute, style::Print};
use std::io::stdout;
mod raw_mode;

fn main() {
    raw_mode::init();

    loop {
        let Ok(evt) = event::read() else {
            continue;
        };
        match evt {
            Event::Mouse(m) if m.kind.is_down() => {
                let _ = execute!(stdout(), cursor::MoveTo(m.column, m.row), Print("●"));
            }
            Event::Key(k) if k.code.is_esc() => break,
            _ => {}
        }
    }

    raw_mode::restore();
}
