use crate::dialog::DialogTree;
use crate::logic::Game;
use crate::ui::{get_player_action, render};
use std::collections::HashMap;

pub struct Gameplay;

impl Gameplay {
    pub fn start(game: &mut Game, dialog: &mut DialogTree) {
        loop {
            render(game);

            if let Some(action) = get_player_action() {
                if !Self::handle_action(action, game, dialog) {
                    break;
                }
            }
        }
    }

    fn handle_action(action: char, game: &mut Game, dialog: &mut DialogTree) -> bool {
        match action.to_ascii_lowercase() {
            'h' => {
                if let Some(_card) = game.player_hit() {
                    dialog.display_dialog("player_actions", "hit", None);

                    let mut vars = HashMap::new();
                    vars.insert(
                        "total".to_string(),
                        Game::calculate_hand_value(&game.player_hand).to_string(),
                    );
                    
                    dialog.display_dialog("player_actions", "player_hand", Some(vars));
                    
                    if game.is_player_bust() {
                        dialog.display_dialog("game_actions", "player_bust", None);
                        return false;
                    }
                }
            }
            
            's' => {
                dialog.display_dialog("player_actions", "stand", None);
                dialog.display_dialog("dealer_actions", "dealer_turn", None);
                game.dealer_turn(dialog);
                return false;
            }
            
            _ => {
                println!("Invalid action. Please choose [H]it, [S]tand, or [Q]uit.");
            }
        }
        
        true
    }
}
