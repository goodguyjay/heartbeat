mod audio;
mod dialog;
mod logic;
mod ui;

use dialog::DialogTree;

fn main() {
    let dialog = DialogTree::load("localization/dialog.json");

    println!("{}", dialog.get("intro", "dealer"));

    println!("{}", dialog.get("intro", "ask_name"));
    let player_name = ui::get_input().trim().to_string();

    println!("\n{}", dialog.get("anna", "introduction"));
    println!("{}", dialog.get("anna", "empathy_hook"));

    let mut game = logic::Game::new();

    loop {
        ui::render(&game);

        if let Some(action) = ui::get_player_action() {
            game.apply_action(&action);

            match action {
                logic::Action::Stimulate => println!("{}", dialog.get("turn", "stimulate")),
                logic::Action::SlowDown => println!("{}", dialog.get("turn", "slow_down")),
                logic::Action::StayStill => println!("{}", dialog.get("turn", "stay_still")),
                logic::Action::Quit => break,
            }
        }

        println!("Processing");
        ui::print_dots(std::time::Duration::from_secs(3));

        ui::clear_console();
    }

    println!("Thanks for playing.");
}
