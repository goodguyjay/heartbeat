mod audio;
mod logic;
mod ui;

fn main() {
    let mut game = logic::Game::new();

    loop {
        ui::render(&game);

        if let Some(action) = ui::get_player_action() {
            if action == logic::Action::Quit {
                break;
            }
            game.apply_action(action);
        }
    }

    println!("Thanks for playing.");
}
