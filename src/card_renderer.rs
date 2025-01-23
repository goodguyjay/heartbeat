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

pub fn render_card_rows(card: &Card) -> Vec<String> {
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

    vec![
        format!("{}╭─────────╮\x1b[0m", color_code),
        format!("{}│ {rank:<2}      │\x1b[0m", color_code, rank = card.rank),
        format!("{}│         │\x1b[0m", color_code),
        format!("{}│    {suit}    │\x1b[0m", color_code, suit = suit_symbol),
        format!("{}│         │\x1b[0m", color_code),
        format!("{}│      {rank:>2} │\x1b[0m", color_code, rank = card.rank),
        format!("{}╰─────────╯\x1b[0m", color_code),
    ]
}

pub fn render_hand(cards: &[Card]) -> String {
    if cards.is_empty() {
        return String::new();
    }

    let card_rows: Vec<Vec<String>> = cards.iter().map(render_card_rows).collect();

    let mut combined_rows = Vec::new();

    for row_idx in 0..card_rows[0].len() {
        let row = card_rows
            .iter()
            .map(|rows| rows[row_idx].clone())
            .collect::<Vec<String>>()
            .join(" ");
        combined_rows.push(row);
    }

    combined_rows.join("\n")
}

pub fn format_color(suit: &str) -> Color {
    match suit {
        "Hearts" | "Diamonds" => Color::Red,
        "Spades" | "Clubs" => Color::White,
        _ => Color::Grey,
    }
}
