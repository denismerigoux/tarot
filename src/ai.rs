use game;
use deck;
use card;

// Each player looks at his game and termines if he takes or not, returns the taker's id
pub fn auctions(players: &Vec<game::Player>) -> i32 {
    let potentials: Vec<(i32, i32)> = players.iter()
        .map(|ref player| (player.id, evaluate_hand_potential(&player.hand)))
        .collect();
    let &(taker_id, _) = match potentials.iter().max_by_key(|&&(id, _)| id) {
        None => panic!("no players"),
        Some(x) => x,
    };
    taker_id
}

pub fn evaluate_hand_potential(hand: &deck::Hand) -> i32 {
    hand.iter().fold(0, |acc, card| acc + card::potential_value(card))
}

pub fn exchange_dog(mut players: &mut Vec<game::Player>,
                    dog: &mut deck::Hand,
                    game: &mut game::Game) {
    let ref mut taker = match players.iter_mut().find(|player| player.id == game.taker_id) {
        None => panic!("cannot find the taker"),
        Some(player) => player,
    };
    // Now we proceed to exchange the cards in the dog
    // We adopt a simple and naïve strategy
    // We let the low cards in the dog and exchange useful for random low others
    let mut new_dog: Vec<(card::Card, i32)> = Vec::new();
    dog.clone()
        .iter()
        .fold((), |_, &card| {
            let to_exchange = match card {
                card::Card::Trump(_) => true,
                card::Card::Face(card::Face { suit: _, symbol }) => {
                    match symbol {
                        card::Symbol::Jack | card::Symbol::Knight | card::Symbol::Queen |
                        card::Symbol::King => true,
                        _ => false,
                    }
                }
                card::Card::Fool => true,
            };
            if to_exchange {
                // Performs the exchange
                dog.remove(&card);
                new_dog.push((card, taker.id));
            }
        });
    // We exchange the remaining cards with cards in the hand of the taker
    dog.clone().iter().fold((), |_, &dog_card| {
        let taker_hand = taker.hand.clone();
        let exchange_card = taker_hand.iter().filter(|&&card| dog_card > card).next();
        match exchange_card {
            None => (),
            Some(&exchange_card) => {
                dog.remove(&dog_card);
                taker.hand.insert(dog_card);
                taker.hand.remove(&exchange_card);
                new_dog.push((exchange_card, taker.id));
            }
        };
    });
    //Then for the remaining cards in the dog we simply transfer them to the new dog
    dog.clone().iter().fold((), |_, &card| {
        dog.remove(&card);
        new_dog.push((card, taker.id));
    });

    // Finaly the new dog goes into the attack's heap
    game.attack_heap.append(&mut new_dog);
}

// Select a card to play among the valid cards
pub fn select_card(_: &game::Player, valid_cards: &deck::Hand, _: &game::Heap) -> card::Card {
    match valid_cards.iter().next() {
        None => panic!("no valid card to play"),
        Some(card) => card.clone(),
    }
}
