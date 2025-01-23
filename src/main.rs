mod audio;
mod card_renderer;
mod dialog;
mod logic;
mod ui;

use crate::logic::Game;
use crate::ui::{clear_console, get_player_action, render};

fn main() {
    let mut game = Game::new();
    game.deal_initial_hands();

    loop {
        clear_console();
        render(&game);
        
        if let Some(action) = get_player_action() {
            match action {
                'h' => {
                    if let Some(card) = game.deck.deal() {
                        game.player_hand.push(card);
                        println!("You drew a card.");
                        
                        let player_value = Game::calculate_hand_value(&game.player_hand);
                        println!("Your total is: {}", player_value);
                        
                        if player_value > 21 {
                            println!("You busted. Dealer wins.");
                            break;
                        }
                    }
                }
                
                's' => {
                    println!("You chose to stand. Dealer's turn.");
                    break;
                }
                
                'q' => {
                    println!("Thanks for playing.");
                    break;
                }
                
                _ => {
                    println!("Invalid action. Please choose [H]it, [S]tand, or [Q]uit.");
                }
            }
        }
    }
}
