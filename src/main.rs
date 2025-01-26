mod audio;
mod card_renderer;
mod dialog;
mod logic;
mod ui;

use crate::dialog::DialogTree;
use crate::logic::Game;
use crate::ui::{clear_console, get_player_action, render};
use std::collections::HashMap;

fn main() {
    let mut dialog = DialogTree::load("localization/dialog.json");

    clear_console();
    dialog.display_dialog("intro", "dealer_introduction", None);

    dialog.display_dialog("intro", "ask_name", None);
    let player_name = ui::get_player_name();

    let mut vars = HashMap::new();
    vars.insert("player_name".to_string(), player_name.clone());

    dialog.display_dialog("intro", "taunt_name", Some(vars));

    let mut game = Game::new();
    game.deal_initial_hands();

    loop {
        // Commented line until I think of a better way to clear the console
        // clear_console();
        render(&game);

        if let Some(action) = get_player_action() {
            match action {
                'h' => {
                    if let Some(card) = game.deck.deal() {
                        game.player_hand.push(card);
                        dialog.display_dialog("player_actions", "hit", None);

                        let player_value = Game::calculate_hand_value(&game.player_hand);
                        println!("debug: player_value: {}", player_value);

                        let mut vars = HashMap::new();
                        vars.insert("total".to_string(), player_value.to_string());
                        dialog.display_dialog("player_actions", "player_hand", Some(vars));

                        if player_value > 21 {
                            dialog.display_dialog("game_actions", "player_bust", None);
                            break;
                        }
                    }
                }

                's' => {
                    dialog.display_dialog("player_actions", "stand", None);
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
