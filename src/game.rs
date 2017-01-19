use params;
use deck;
use card;
use std::collections::BTreeSet;

type Heap = Vec<card::Card>;

pub fn start_game(dog: deck::Hand, hands: [deck::Hand; params::NUMBER_OF_PLAYERS]) {
    let cards_per_player = hands[0].1.len();
    //Check if all players have same number of cards
    for i in 1..params::NUMBER_OF_PLAYERS {
        if cards_per_player != hands[i].1.len() {
            panic!("the players don't have the same number of cards")
        }
    }
    //If yes start the game
    let starting_player = 0;
    (1..cards_per_player).fold((starting_player, hands), |(starting_player, hands), _| {
        //For each turn
        let mut new_hands: [deck::Hand; params::NUMBER_OF_PLAYERS] = [(0, BTreeSet::new()),
                                                                      (1, BTreeSet::new()),
                                                                      (2, BTreeSet::new()),
                                                                      (3, BTreeSet::new())];
        let end_heap = hands.into_iter().enumerate().fold(Vec::new(), |heap, (i, hand)| {
            // For each player
            let (new_hand, new_heap) = play_card(hand.clone(), heap);
            new_hands[i - starting_player % params::NUMBER_OF_PLAYERS] = new_hand;
            new_heap
        });
        let new_starting_player = winning_card(end_heap.clone());
        println!("Les joueurs ont joué : {:?}", end_heap);
        (new_starting_player, new_hands)
    });
}

fn play_card((player, hand): deck::Hand, cards_played: Heap) -> (deck::Hand, Heap) {
    let valid_cards = valid_cards((player, hand.clone()), cards_played.clone());
    let card_to_play = select_card(valid_cards, cards_played.clone());
    println!("Card to play: {:?}", card_to_play.clone());
    let mut new_hand = hand.clone();
    new_hand.take(&card_to_play);
    let mut new_cards_played = cards_played.clone();
    new_cards_played.push(card_to_play);
    ((player, new_hand), new_cards_played)
}

fn valid_cards(hand: deck::Hand, cards_played: Heap) -> deck::Hand {
    hand
}

fn select_card((player, valid_cards): deck::Hand, cards_played: Heap) -> card::Card {
    match valid_cards.into_iter().next() {
        Some(card) => card,
        None => panic!("there is no valid card to play"),
    }
}

fn winning_card(cards: Heap) -> usize {
    0
}
