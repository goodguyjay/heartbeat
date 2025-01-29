use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
};

use crate::card_renderer::render_hand;
use crate::logic::Game;
use std::io::{stdin, stdout, Write};

pub fn render(game: &Game) {
    println!("Dealer's hand:");
    println!("{}", render_hand(&game.dealer_hand));

    println!("Your hand:");
    println!("{}", render_hand(&game.player_hand));

    println!("\nChoose an action:");
    println!("[H] Hit");
    println!("[S] Stand");
    println!("[Q] Quit");
}

pub fn clear_console() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
}

pub fn get_player_name() -> String {
    let mut input = String::new();
    let mut stdout = stdout();
    
    print!("> ");
    stdout.flush().unwrap();
    
    stdin().read_line(&mut input).unwrap();
    
    input.trim().to_string()
}

pub fn get_player_action() -> Option<char> {
    loop {
        match read().unwrap() {
            Event::Key(key_event) => {
                return match key_event.code {
                    KeyCode::Char('h') => Some('h'),
                    KeyCode::Char('s') => Some('s'),
                    KeyCode::Char('q') => Some('q'),
                    KeyCode::Enter => continue,
                    _ => None,
                }
            }
            _ => continue,
        }
    }
}
