mod card;
mod deck;

extern crate rand;

fn main() {
    let mut deck = deck::new_deck();
    for card in deck {
        println!("{}", card)
    }
}
