use params;
use deck;
use card;
use ai;

pub type Heap = Vec<(card::Card, i32)>;

#[derive(Clone)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub hand: deck::Hand,
}

pub struct Game {
    pub taker_id: i32,
    pub attack_heap: Heap,
    pub defense_heap: Heap,
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

//Plays the game and returns the tuple (attack_heap,defense_heap)
pub fn play_game(mut dog: &mut deck::Hand, mut players: &mut Vec<Player>) -> Game {
    let cards_per_player = players[0].hand.len();
    // Check if all players have same number of cards
    for i in 1..params::NUMBER_OF_PLAYERS {
        if cards_per_player != players[i].hand.len() {
            panic!("the players don't have the same number of cards")
        }
    }

    // Determine the attack and defense
    let taker_id = ai::auctions(&players);
    let mut game = Game {
        taker_id: taker_id,
        attack_heap: Vec::new(),
        defense_heap: Vec::new(),
    };
    println!(">>>> {} prend le chien <<<<<",
             match players.iter().find(|player| player.id == taker_id) {
                 None => panic!("cannot find the player who won the round"),
                 Some(player) => &player.name,
             });

    ai::exchange_dog(&mut players, &mut dog, &mut game);

    let mut starting_player_id = 0;
    let mut fool_on_standby: Option<(card::Card, i32)> = None;
    println!("===== Début de la partie ! =====");
    for _ in 0..cards_per_player {
        // For each round of the game
        let mut cards_played: Heap = Vec::new();
        sort_by_playing_order(&mut players, starting_player_id);
        for player in players.iter_mut() {
            // Each player plays a card
            play_card(player, &mut cards_played)
        }
        // Decide who has won
        starting_player_id = winning_player(&mut cards_played);
        {
            // Printing winner
            let last_card_played = match cards_played.first() {
                None => panic!("no card played this round"),
                Some(card) => card,
            };
            println!(">>>>> {} remporte le pli avec la carte {} ! <<<<<",
                     match players.iter().find(|player| player.id == last_card_played.1) {
                         None => panic!("cannot find the player who won the round"),
                         Some(player) => &player.name,
                     },
                     last_card_played.0);
        }

        // Before giving cards to the winner's heap, must first apply special rule for the fool
        match fool_on_standby {
            None => (),
            Some(fool) => {
                // The player who played the fool couldn't exchange cards in a prior tour,
                // trying now to do so
                cards_played.push(fool)
            }
        }
        fool_on_standby = deal_with_fool(&mut cards_played, &mut game, starting_player_id);

        // Giving cards to the winner's heap
        if starting_player_id == game.taker_id {
            game.attack_heap.append(&mut cards_played);
        } else {
            game.defense_heap.append(&mut cards_played);
        }
    }
    // End of game
    match fool_on_standby {
        None => (),
        Some((fool, id)) => {
            // The player never could exchange his fool, we return it to him
            if id == game.taker_id {
                game.attack_heap.push((fool, id));
            } else {
                game.defense_heap.push((fool, id));
            }
        }
    }
    game
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
    let card = ai::select_card(player, &valid_cards, cards_played);
    player.hand.remove(&card);
    cards_played.push((card, player.id));
    println!("{} joue {}.", player.name, card);
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
                    let max_trump_played = match max_trump(cards_played) {
                        Some(trump) => trump,
                        None => panic!("this shouldn't happen"),
                    };
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
                        // Play a trump higher than those alreay played
                        let max_trump_played = max_trump(cards_played);
                        let valid_trumps: deck::Hand = valid_hand.iter()
                            .map(|&card| card.clone())
                            .filter(|&card| match card {
                                card::Card::Trump(trump1) => {
                                    match max_trump_played {
                                        Some(trump2) => trump1 > trump2,
                                        None => true,
                                    }
                                }
                                _ => false,
                            })
                            .collect();
                        if !valid_trumps.is_empty() {
                            return valid_trumps;
                        } else {
                            // Play any other trump
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
}

// Returns the max trump in a heap of played cards
fn max_trump(cards_played: &Heap) -> Option<card::Trump> {
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
    max_trump
}

// Determine who won the round
fn winning_player(cards: &mut Heap) -> i32 {
    cards.sort_by(|&(card1, _), &(card2, _)| card2.cmp(&card1));
    match cards.first() {
        Some(&(_, id)) => id,
        None => panic!("no cards have been played"),
    }
}

// Proceeds to exchange the fool against a lower card of the opponent's heap if necessary
fn deal_with_fool(mut cards_played: &mut Heap,
                  mut game: &mut Game,
                  winning_player_id: i32)
                  -> Option<(card::Card, i32)> {
    let mut fool_index: Option<usize> = None;
    let mut fool_on_standby: Option<(card::Card, i32)> = None;
    for (index, &(card_played, _)) in cards_played.iter().enumerate() {
        if card_played == card::Card::Fool {
            fool_index = Some(index);
            break;
        }
    }
    match fool_index {
        None => (),
        Some(index) => {
            let fool = cards_played.remove(index);
            if fool.1 == game.taker_id && winning_player_id != game.taker_id {
                // The attack played the fool and lost => exchange
                let attack_copy = game.attack_heap.clone();
                let card_to_exchange = attack_copy.iter()
                    .enumerate()
                    .filter(|&(_, &(card, _))| match card {
                        card::Card::Trump(_) => false,
                        card::Card::Face(card::Face { suit: _, symbol }) => {
                            match symbol {
                                card::Symbol::Jack | card::Symbol::Knight |
                                card::Symbol::Queen | card::Symbol::King => false,
                                _ => true,
                            }
                        }
                        card::Card::Fool => false,
                    })
                    .next();
                match card_to_exchange {
                    Some((index, &card_to_exchange)) => {
                        game.attack_heap.remove(index);
                        game.defense_heap.push(card_to_exchange);
                        game.attack_heap.push(fool);
                    }
                    None => fool_on_standby = Some(fool),
                };
            } else if fool.1 != game.taker_id && winning_player_id == game.taker_id {
                // The defense played the fool and lost => exchange
                let defense_copy = game.defense_heap.clone();
                let card_to_exchange = defense_copy.iter()
                    .enumerate()
                    .filter(|&(_, &(card, _))| match card {
                        card::Card::Trump(_) => false,
                        card::Card::Face(card::Face { suit: _, symbol }) => {
                            match symbol {
                                card::Symbol::Jack | card::Symbol::Knight |
                                card::Symbol::Queen | card::Symbol::King => false,
                                _ => true,
                            }
                        }
                        card::Card::Fool => false,
                    })
                    .next();
                match card_to_exchange {
                    Some((index, &card_to_exchange)) => {
                        game.defense_heap.remove(index);
                        game.attack_heap.push(card_to_exchange);
                        game.defense_heap.push(fool);
                    }
                    None => fool_on_standby = Some(fool),
                };
            } else {
                // Normal case, the fool shouldn't be exchanged
                cards_played.insert(index, fool);
            }
        }
    };
    fool_on_standby
}
