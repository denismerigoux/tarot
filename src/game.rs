use params;
use deck;
use card;

type Heap = Vec<(card::Card, i32)>;

#[derive(Clone)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub hand: deck::Hand,
}

// Empty the hands and names vector and creates a player vector instead
pub fn initialize_players(hands: &mut Vec<deck::Hand>, names: &mut Vec<String>) -> Vec<Player> {
    if hands.len() != names.len() {}
    let mut counter = 0;
    let mut players = Vec::new();
    for _ in 0..hands.len() {
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

pub fn start_game(_: &deck::Hand, mut players: &mut Vec<Player>) {
    let cards_per_player = players[0].hand.len();
    //Check if all players have same number of cards
    for i in 1..params::NUMBER_OF_PLAYERS {
        if cards_per_player != players[i].hand.len() {
            panic!("the players don't have the same number of cards")
        }
    }
    let mut starting_player_id = 1;
    for _ in 0..cards_per_player {
        // For each round of the game
        let mut cards_played: Heap = Vec::new();
        sort_by_playing_order(&mut players, starting_player_id);
        for player in players.iter_mut() {
            // For each player
            play_card(player, &mut cards_played)
        }
        // Decide who has won
        starting_player_id = winning_player(&mut cards_played);
    }
}

fn sort_by_playing_order(mut players: &mut Vec<Player>, starting_player_id: i32) {
    players.sort_by(|p1, p2| {
        (p1.id + starting_player_id % params::NUMBER_OF_PLAYERS as i32)
            .cmp(&(p2.id + starting_player_id % params::NUMBER_OF_PLAYERS as i32))
    });
}

fn play_card(player: &mut Player, cards_played: &mut Heap) {
    let valid_cards = valid_cards(&player.hand, cards_played);
    let card = select_card(player, &valid_cards, cards_played);
    player.hand.remove(&card);
    cards_played.push((card, player.id));
}

fn valid_cards(hand: &deck::Hand, cards_played: &Heap) -> deck::Hand {
    unimplemented!();
}

fn select_card(player: &Player, valid_cards: &deck::Hand, cards_played: &Heap) -> card::Card {
    unimplemented!();
}

fn winning_player(cards: &mut Heap) -> i32 {
    cards.sort_by(|&(card1, _), &(card2, _)| card1.cmp(&card2));
    match cards.first() {
        Some(&(_, id)) => id,
        None => panic!("no cards have been played"),
    }
}
