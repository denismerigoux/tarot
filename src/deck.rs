use card;
use rand::{thread_rng, Rng};

pub fn new_deck() -> Vec<card::Card> {
    let mut deck: Vec<card::Card> = Vec::new();
    for suit in card::Suit::values() {
        for symbol in card::Symbol::values() {
            deck.push(card::Card::Face(card::Face {
                suit: suit,
                symbol: symbol,
            }))
        }
    }
    for trump in card::Trump::values() {
        deck.push(card::Card::Trump(trump))
    }
    deck.push(card::Card::Fool);
    let mut rng = thread_rng();
    rng.shuffle(&mut deck);
    deck
}
