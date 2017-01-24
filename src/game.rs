use params;
use deck;
use card;
use std::collections::BTreeSet;

type Heap = Vec<card::Card>;
pub struct Player {
    pub id: i32,
    pub name: String,
    pub hand: deck::Hand,
}

pub fn initialize_players(hands: &mut Vec<deck::Hand>, names: &mut Vec<String>) -> Vec<Player> {
    if hands.len() != names.len() {}
    let mut counter = 0;
    let mut players = Vec::new();
    for i in 0..hands.len() {
        let name = match names.pop() {
            Some(name) => name,
            None => panic!("the number of hands is different from the number of players"),
        };
        let hand = match hands.pop() {
            Some(hand) => hand,
            None => panic!("the number of hands is different from the number of players"),
        };
        players.push(Player {
            id: counter,
            name: name,
            hand: hand,
        });
        counter += 1;
    }
    players
}

pub fn start_game(dog: &deck::Hand, players: &mut Vec<Player>) {
    let cards_per_player = players[0].hand.len();
    //Check if all players have same number of cards
    for i in 1..params::NUMBER_OF_PLAYERS {
        if cards_per_player != players[i].hand.len() {
            panic!("the players don't have the same number of cards")
        }
    }
    for i in 0..cards_per_player {
        for player in players.iter_mut() {}
    }
    unimplemented!();
}

fn play_card(player: &mut Player, cards_played: &mut Heap) {
    unimplemented!();
}

fn valid_cards(hand: &deck::Hand, cards_played: &Heap) -> deck::Hand {
    unimplemented!();
}

fn select_card(player: &Player, cards_played: &Heap) -> card::Card {
    unimplemented!();
}

fn winning_card(cards: &Heap) -> usize {
    unimplemented!();
}
