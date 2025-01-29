use crate::card_renderer::{format_color, Card};
use crate::dealer::{DealerAI, DealerAction};
use crate::dialog::DialogTree;
use crate::ui;
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
    pub dealer: DealerAI,
    pub deck: Deck,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        Self {
            player_hand: Vec::new(),
            dealer: DealerAI::new(),
            deck,
        }
    }

    pub fn deal_initial_hands(&mut self) {
        for _ in 0..2 {
            self.player_hand.push(self.deck.deal().unwrap());
            self.dealer.hand.push(self.deck.deal().unwrap());
        }
    }

    pub fn calculate_hand_value(hand: &[Card]) -> u8 {
        let mut value = 0;
        let mut aces = 0;

        for card in hand {
            value += match card.rank.as_str() {
                "A" => 11,
                "K" | "Q" | "J" => 10,
                "10" => 10,
                "9" => 9,
                "8" => 8,
                "7" => 7,
                "6" => 6,
                "5" => 5,
                "4" => 4,
                "3" => 3,
                "2" => 2,
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

    pub fn player_hit(&mut self) -> Option<&Card> {
        if let Some(card) = self.deck.deal() {
            self.player_hand.push(card);
            self.player_hand.last()
        } else {
            None
        }
    }

    pub fn is_player_bust(&self) -> bool {
        Self::calculate_hand_value(&self.player_hand) > 21
    }

    pub fn dealer_turn(&mut self, dialog: &mut DialogTree) {
        loop {
            match self.dealer.take_action(&mut self.deck) {
                DealerAction::Hit => {
                    ui::render(self);
                    dialog.display_dialog("game_actions", "dealer_hit", None);
                }

                DealerAction::Stand => {
                    ui::render(self);
                    dialog.display_dialog("game_actions", "dealer_stand", None);
                    break;
                }
            }
        }

        let player_value = Game::calculate_hand_value(&self.player_hand);
        let dealer_value = Game::calculate_hand_value(&self.dealer.hand);

        if dealer_value > 21 {
            println!("dealer busts. placeholder dialog");
        } else if player_value > dealer_value {
            println!("you win. placeholder dialog");
        } else if player_value < dealer_value {
            println!("dealer wins. placeholder dialog");
        } else {
            println!("tie. placeholder dialog");
        }
    }
}
