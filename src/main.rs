mod audio;
mod card_renderer;
mod dealer;
mod dialog;
mod gameplay;
mod logic;
mod ui;

use crate::dialog::DialogTree;
use crate::gameplay::Gameplay;
use crate::logic::Game;
use crate::ui::clear_console;
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

    Gameplay::start(&mut game, &mut dialog);
}
