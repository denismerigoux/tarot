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
    let mut starting_player_id = 0;
    println!("===== Début de la partie ! =====");
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
        let last_card_played = match cards_played.first() {
            None => panic!("no card played this round"),
            Some(card) => card,
        };
        println!(">>>>> Le joueur {} remporte le pli avec la carte {} ! <<<<<",
                 last_card_played.1,
                 last_card_played.0);
    }
}

// Keeps the order of the game but puts starting player first
fn sort_by_playing_order(mut players: &mut Vec<Player>, starting_player_id: i32) {
    // Implementation via a sort with a complicated comparison function
    // that assume that ids go from 0 to params::NUMBER_OF_PLAYERS-1
    players.sort_by(|p1, p2| {
        ((params::NUMBER_OF_PLAYERS as i32 + p1.id - starting_player_id) %
         params::NUMBER_OF_PLAYERS as i32)
            .cmp(&((params::NUMBER_OF_PLAYERS as i32 + p2.id - starting_player_id) %
                   params::NUMBER_OF_PLAYERS as i32))
    });
}

fn play_card(player: &mut Player, cards_played: &mut Heap) {
    let valid_cards = valid_cards(&player, cards_played);
    let card = select_card(player, &valid_cards, cards_played);
    player.hand.remove(&card);
    cards_played.push((card, player.id));
    println!("Le joueur {} joue {}.", player.id, card);
}

fn valid_cards(player: &Player, cards_played: &Heap) -> deck::Hand {
    let valid_hand = player.hand.clone();
    match cards_played.first() {
        // If first to play then every card is valid
        None => return valid_hand,
        Some(&(card, _)) => {
            match card {
                // If fool played first you can play what you want
                card::Card::Fool => {
                    let mut new_cards_played = cards_played.clone();
                    let _ = new_cards_played.remove(0);
                    return valid_cards(player, &new_cards_played);
                }
                // If it's a trump you can play only higher trumps,
                // otherwise lower trumps, otherwise anything
                card::Card::Trump(_) => {
                    let max_trump_played = max_trump(cards_played);
                    let higher_trumps: deck::Hand = valid_hand.iter()
                        .map(|&card| card.clone())
                        .filter(|&card| match card {
                            card::Card::Trump(this_trump) => max_trump_played < this_trump,
                            _ => false,
                        })
                        .collect();
                    if !higher_trumps.is_empty() {
                        return higher_trumps;
                    } else {
                        // Play a lower trump
                        let lower_trumps: deck::Hand = valid_hand.iter()
                            .map(|&card| card.clone())
                            .filter(|&card| match card {
                                card::Card::Trump(this_trump) => max_trump_played > this_trump,
                                _ => false,
                            })
                            .collect();
                        if !lower_trumps.is_empty() {
                            return lower_trumps;
                        } else {
                            // Play anaything
                            return valid_hand;
                        }
                    }
                }
                // If it's a face you can only play a face of the same suit, otherwise a trump,
                // otherwise anything
                card::Card::Face(card::Face { suit, symbol: _ }) => {
                    let same_suit_faces: deck::Hand = valid_hand.iter()
                        .map(|&card| card.clone())
                        .filter(|&card| match card {
                            card::Card::Face(card::Face { suit: this_suit, symbol: _ }) => {
                                suit == this_suit
                            }
                            _ => false,
                        })
                        .collect();
                    if !same_suit_faces.is_empty() {
                        return same_suit_faces;
                    } else {
                        // Play a trump
                        let trumps: deck::Hand = valid_hand.iter()
                            .map(|&card| card.clone())
                            .filter(|&card| match card {
                                card::Card::Trump(_) => true,
                                _ => false,
                            })
                            .collect();
                        if !trumps.is_empty() {
                            return trumps;
                        } else {
                            //Play anything
                            return valid_hand;
                        }
                    }
                }
            }
        }
    }
}

// Returns the max trump in a heap of played cards, panics if no trump
fn max_trump(cards_played: &Heap) -> card::Trump {
    let mut max_trump: Option<card::Trump> = None;
    for &(card, _) in cards_played.iter() {
        match card {
            card::Card::Trump(trump) => {
                match max_trump {
                    None => max_trump = Some(trump),
                    Some(max) => {
                        if max < trump {
                            max_trump = Some(trump);
                        }
                    }
                }
            }
            _ => (),
        }
    }
    match max_trump {
        None => panic!("no trump, cannot find max"),
        Some(max) => max,
    }
}

// Select a card to play among the valid cards
fn select_card(_: &Player, valid_cards: &deck::Hand, _: &Heap) -> card::Card {
    match valid_cards.iter().next() {
        None => panic!("no valid card to play"),
        Some(card) => card.clone(),
    }
}

// Determine who won the round
fn winning_player(cards: &mut Heap) -> i32 {
    cards.sort_by(|&(card1, _), &(card2, _)| card2.cmp(&card1));
    match cards.first() {
        Some(&(_, id)) => id,
        None => panic!("no cards have been played"),
    }
}
