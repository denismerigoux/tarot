mod card;
mod deck;

extern crate rand;

fn main() {
    let deck = deck::new_deck();
    let (dog, hands) = deck::distribute_cards(deck);
    println!("-> Cartes du chien :");
    for card in dog.iter() {
        println!("{}", card);
    }
    for i in 0..(deck::NUMBER_OF_PLAYERS) {
        println!("-> Cartes du joueur {} :", i + 1);
        for card in hands[i].iter() {
            println!("{}", card);
        }
    }
}
