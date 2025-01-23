use crossterm::style::{Color, Stylize};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Card {
    pub rank: String,
    pub suit: String,
    pub color: Color,
}

impl Card {
    pub fn new(rank: &str, suit: &str, color: Color) -> Self {
        Self {
            rank: rank.to_string(),
            suit: suit.to_string(),
            color,
        }
    }
}

pub fn render_card(card: &Card) -> String {
    let suit_symbol = match card.suit.as_str() {
        "Spades" => "♠",
        "Hearts" => "♥",
        "Diamonds" => "♦",
        "Clubs" => "♣",
        _ => "?",
    };

    let color_code = match card.color {
        Color::Red => "\x1b[31m",   // Red
        Color::White => "\x1b[37m", // White/Black
        _ => "\x1b[0m",             // Reset
    };

    format!(
        "{}╭─────────╮\x1b[0m\n\
         {}│ {rank:<2}      │\x1b[0m\n\
         {}│         │\x1b[0m\n\
         {}│    {suit}    │\x1b[0m\n\
         {}│         │\x1b[0m\n\
         {}│      {rank:>2} │\x1b[0m\n\
         {}╰─────────╯\x1b[0m",
        color_code,
        color_code,
        color_code,
        color_code,
        color_code,
        color_code,
        color_code,
        rank = card.rank,
        suit = suit_symbol
    )
}

pub fn render_hand(cards: &[Card]) -> String {
    cards
        .iter()
        .map(render_card)
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn format_color(suit: &str) -> Color {
    match suit {
        "Hearts" | "Diamonds" => Color::Red,
        "Spades" | "Clubs" => Color::White,
        _ => Color::Grey,
    }
}
