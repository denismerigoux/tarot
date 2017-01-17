mod card;

fn main() {
    let card1: card::Card = card::Card::Face(card::Face {
        suit: card::Suit::Hearts,
        symbol: card::Symbol::Seven,
    });
    let card2: card::Card = card::Card::Trump(card::Trump::Two);
    println!("{} et {}", card1, card2);
}
