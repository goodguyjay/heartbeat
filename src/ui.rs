use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};

use std::io::{stdout, Write};

use crate::logic::{Action, Game};

pub fn render(game: &Game) {
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All)).unwrap();

    println!("Current Heartbeat: {}", game.heartbeat);

    println!("\nChoose an action:");
    println!("[S] Stimulate (+5 to +15)");
    println!("[D] Slow down (-5 to -15)");
    println!("[Q] Quit");
}

pub fn get_player_action() -> Option<Action> {
    match read().unwrap() {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Char('s') => Some(Action::Stimulate),
            KeyCode::Char('d') => Some(Action::SlowDown),
            KeyCode::Char('q') => Some(Action::Quit),
            _ => None,
        },
        _ => None,
    }
}
