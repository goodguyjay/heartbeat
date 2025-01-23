use crate::card_renderer::{format_color, Card};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks = vec![
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];

        let mut cards = Vec::new();

        for suit in suits {
            let color = format_color(suit);
            for rank in ranks.iter() {
                cards.push(Card::new(rank, suit, color));
            }
        }

        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

pub struct Game {
    pub player_hand: Vec<Card>,
    pub dealer_hand: Vec<Card>,
    pub deck: Deck
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        
        Self {
            player_hand: Vec::new(),
            dealer_hand: Vec::new(),
            deck
        }
    }
    
    pub fn deal_initial_hands(&mut self) {
        for _ in 0..2 {
            self.player_hand.push(self.deck.deal().unwrap());
            self.dealer_hand.push(self.deck.deal().unwrap());
        }
    }
    
    pub fn calculate_hand_value(hand: &[Card]) -> u8 {
        let mut value = 0;
        let mut aces = 0;
        
        for card in hand {
            value += match card.rank.as_str() {
                "A" => 11,
                "K" | "Q" | "J" => 10,
                _ => card.rank.parse::<u8>().unwrap_or(0),
            };
            
            if card.rank == "A" {
                aces += 1;
            }
        }

        while value > 21 && aces > 0 {
            value -= 10;
            aces -= 1;
        }
            
        value
    }
}
