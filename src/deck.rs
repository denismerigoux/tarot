use card;
use params;
use rand::{thread_rng, Rng};
use std::collections::BTreeSet;

pub type Deck = [card::Card; params::DECK_SIZE];
pub type Hand = (i32, BTreeSet<card::Card>);

// Creates a deck_size cards tarot deck, shuffled
pub fn new_deck() -> Deck {
    let mut deck: [card::Card; params::DECK_SIZE] = [card::Card::Fool; params::DECK_SIZE];
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
pub fn distribute_cards(deck: Deck) -> (Hand, [Hand; params::NUMBER_OF_PLAYERS]) {
    let (_, mut dog): Hand = (-1, BTreeSet::new());
    let mut hands: [Hand; params::NUMBER_OF_PLAYERS] =
        [(0, BTreeSet::new()), (1, BTreeSet::new()), (2, BTreeSet::new()), (3, BTreeSet::new())];
    for i in 0..params::DECK_SIZE {
        if i < params::DOG_SIZE {
            dog.insert(deck[i]);
        } else {
            hands[(i / params::CARDS_PER_DISTRIBUTION) % params::NUMBER_OF_PLAYERS]
                .1
                .insert(deck[i]);
        }
    }
    ((-1, dog), hands)
}
