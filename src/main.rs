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
    let (dog, mut hands) = deck::distribute_cards(&deck);
    println!("-> Cartes du chien :");
    for card in dog.iter() {
        println!("{}", card);
    }
    for hand in hands.iter() {
        println!("-> Cartes du joueur {}:", 0);
        for card in hand.iter() {
            println!("{}", card);
        }
    }
    let mut names: Vec<String> =
        vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()];
    let mut players = game::initialize_players(&mut hands, &mut names);
    game::start_game(&dog, &mut players);
}
