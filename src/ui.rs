use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};

use crate::logic::{Action, Game};
use std::io::{stdin, stdout, Write};
use std::time::Duration;

pub fn render(game: &Game) {
    println!("Current Heartbeat: {}", game.heartbeat);
    println!("\nChoose an action:");
    println!("[S] Stimulate (+5 to +15)");
    println!("[D] Slow down (-5 to -15)");
    println!("[Q] Quit");
}

pub fn clear_console() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
}

pub fn print_dots(duration: Duration) {
    let dot_count = 3;
    let interval = duration / dot_count;

    for _ in 0..dot_count {
        print!(".");
        stdout().flush().unwrap();
        std::thread::sleep(interval);
    }

    println!();
}

pub fn get_player_action() -> Option<Action> {
    loop {
        match read().unwrap() {
            Event::Key(key_event) => {
                return match key_event.code {
                    KeyCode::Char('s') => Some(Action::Stimulate),
                    KeyCode::Char('d') => Some(Action::SlowDown),
                    KeyCode::Char('q') => Some(Action::Quit),
                    KeyCode::Enter => continue,
                    _ => None,
                }
            }
            _ => continue,
        }
    }
}

pub fn get_input() -> String {
    let mut input = String::new();
    print!(">");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    input
}
