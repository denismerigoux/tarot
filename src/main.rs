mod card;
mod deck;
mod params;
mod game;

extern crate rand;

pub const DECK_SIZE: usize = 78;
pub const NUMBER_OF_PLAYERS: usize = 4;
pub const CARDS_PER_DISTRIBUTION: usize = 3;

fn main() {
    let deck = deck::new_deck();
    let ((_, dog), hands) = deck::distribute_cards(deck);
    println!("-> Cartes du chien :");
    for card in dog.iter() {
        println!("{}", card);
    }
    for &(number, ref hand) in hands.into_iter() {
        println!("-> Cartes du joueur {}:", number + 1);
        for card in hand.iter() {
            println!("{}", card);
        }
    }
    game::start_game((-1, dog), hands);
}
