use crate::card_renderer::Card;
use crate::logic::{Deck, Game};

pub struct DealerAI {
    pub hand: Vec<Card>,
}

impl DealerAI {
    pub fn new() -> Self {
        Self { hand: Vec::new() }
    }

    pub fn take_action(&mut self, deck: &mut Deck) -> DealerAction {
        let hand_value = Game::calculate_hand_value(&self.hand);

        if hand_value < 17 {
            if let Some(card) = deck.deal() {
                self.hand.push(card);
                return DealerAction::Hit;
            }
        }

        DealerAction::Stand
    }
}

pub enum DealerAction {
    Hit,
    Stand,
}
