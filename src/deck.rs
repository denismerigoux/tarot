use card;
use rand::{thread_rng, Rng};
use std::collections::BTreeSet;

pub const DECK_SIZE: usize = 78;
pub const NUMBER_OF_PLAYERS: usize = 4;
pub const CARDS_PER_DISTRIBUTION: usize = 3;
pub type Deck = [card::Card; DECK_SIZE];
pub type Hand = BTreeSet<card::Card>;


// Creates a deck_size cards tarot deck, shuffled
pub fn new_deck() -> Deck {
    let mut deck: [card::Card; DECK_SIZE] = [card::Card::Fool; DECK_SIZE];
    let mut counter = 0;
    for suit in card::Suit::values() {
        for symbol in card::Symbol::values() {
            deck[counter] = card::Card::Face(card::Face {
                suit: suit,
                symbol: symbol,
            });
            counter += 1
        }
    }
    for trump in card::Trump::values() {
        deck[counter] = card::Card::Trump(trump);
        counter += 1
    }
    deck[counter] = card::Card::Fool;
    let mut rng = thread_rng();
    rng.shuffle(&mut deck);
    deck
}

// Distributes the deck between the four players and the dog
pub fn distribute_cards(deck: Deck) -> (Hand, [Hand; 4]) {
    let mut dog: Hand = BTreeSet::new();
    let mut hands: [Hand; NUMBER_OF_PLAYERS] =
        [BTreeSet::new(), BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];
    for i in 0..(DECK_SIZE - 1) {
        if i <= 5 {
            dog.insert(deck[i]);
        } else {
            hands[(i / CARDS_PER_DISTRIBUTION) % NUMBER_OF_PLAYERS].insert(deck[i]);
        }
    }
    (dog, hands)
}
