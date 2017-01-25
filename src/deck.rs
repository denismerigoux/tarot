use card;
use params;
use rand::{thread_rng, Rng};
use std::collections::BTreeSet;

pub type Deck = Vec<card::Card>;
pub type Hand = BTreeSet<card::Card>;

// Creates a tarot deck, shuffled
pub fn new_deck() -> Deck {
    let mut deck: Vec<card::Card> = Vec::new();
    for suit in card::Suit::values() {
        for symbol in card::Symbol::values() {
            deck.push(card::Card::Face(card::Face {
                suit: suit,
                symbol: symbol,
            }));
        }
    }
    for trump in card::Trump::values() {
        deck.push(card::Card::Trump(trump));
    }
    deck.push(card::Card::Fool);
    let mut rng = thread_rng();
    rng.shuffle(&mut deck);
    deck
}

// Distributes the deck between the four players and the dog
pub fn distribute_cards(deck: &Deck) -> (Hand, Vec<Hand>) {
    let mut dog: Hand = BTreeSet::new();
    let mut hands: Vec<Hand> = Vec::new();
    for _ in 0..(params::NUMBER_OF_PLAYERS) {
        hands.push(BTreeSet::new())
    }
    for i in 0..deck.len() {
        if i < params::DOG_SIZE {
            dog.insert(deck[i]);
        } else {
            hands[(i / params::CARDS_PER_DISTRIBUTION) % params::NUMBER_OF_PLAYERS].insert(deck[i]);
        }
    }
    (dog, hands)
}
