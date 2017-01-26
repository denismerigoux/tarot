use game;
use card;

pub struct GameScores {
    pub attack_score: f32,
    pub defense_score: f32,
    pub attack_bouts_number: i32,
}

pub fn compute_game_scores(game: &game::Game) -> GameScores {
    let mut game_scores: GameScores = GameScores {
        attack_score: 0 as f32,
        defense_score: 0 as f32,
        attack_bouts_number: 0,
    };
    for &(card, _) in game.attack_heap.iter() {
        game_scores.attack_score += card_score(&card);
        match card {
            card::Card::Fool |
            card::Card::Trump(card::Trump::One) |
            card::Card::Trump(card::Trump::TwentyOne) => game_scores.attack_bouts_number += 1,
            _ => (),
        }
    }
    for &(card, _) in game.defense_heap.iter() {
        game_scores.defense_score += card_score(&card);
    }
    game_scores
}

fn card_score(&card: &card::Card) -> f32 {
    match card {
        card::Card::Face(card::Face { suit: _, symbol }) => {
            match symbol {
                card::Symbol::Jack => 1.5,
                card::Symbol::Knight => 2.5,
                card::Symbol::Queen => 3.5,
                card::Symbol::King => 4.5,
                _ => 0.5,
            }
        }
        card::Card::Fool |
        card::Card::Trump(card::Trump::One) |
        card::Card::Trump(card::Trump::TwentyOne) => 4.5,
        _ => 0.5,
    }
}
