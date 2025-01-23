mod audio;
mod card_renderer;
mod dialog;
mod logic;
mod ui;

use card_renderer::{format_color, render_hand, Card};
use dialog::DialogTree;

fn main() {
    let card1 = Card::new("A", "Spades", format_color("Spades"));
    let card2 = Card::new("10", "Hearts", format_color("Hearts"));
    let card3 = Card::new("K", "Diamonds", format_color("Diamonds"));
    let player_hand = vec![card1, card2, card3];

    println!("\nYour hand:");
    println!("{}", render_hand(&player_hand));
}
