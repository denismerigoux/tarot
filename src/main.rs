mod card;
mod deck;
mod params;
mod game;
mod ai;
mod scoring;

extern crate rand;

pub const DECK_SIZE: usize = 78;
pub const NUMBER_OF_PLAYERS: usize = 4;
pub const CARDS_PER_DISTRIBUTION: usize = 3;

fn main() {
    let mut names: Vec<String> =
        vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()];
    let deck = deck::new_deck();
    let (mut dog, mut hands) = deck::distribute_cards(&deck);
    let mut players = game::initialize_players(&mut hands, &mut names);
    println!("-> Cartes du chien :");
    for card in dog.iter() {
        println!("{}", card);
    }
    for player in players.iter() {
        println!("-> Cartes du joueur {}:", player.name);
        for card in player.hand.iter() {
            println!("{}", card);
        }
    }
    let game = game::play_game(&mut dog, &mut players);
    println!("===== Fin de la partie ! =====");
    let game_scores = scoring::compute_game_scores(&game);
    println!("Score de l'attaque ({}) : {} ({} bouts), score de la défense : {}",
             match players.iter().find(|player| player.id == game.taker_id) {
                 None => panic!("cannot find the player who won the round"),
                 Some(player) => &player.name,
             },
             game_scores.attack_score,
             game_scores.attack_bouts_number,
             game_scores.defense_score);
}
