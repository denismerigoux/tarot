use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Suit::Clubs => write!(f, "trèfle"),
            &Suit::Diamonds => write!(f, "carreau"),
            &Suit::Hearts => write!(f, "cœur"),
            &Suit::Spades => write!(f, "pique"),
        }
    }
}

impl Suit {
    pub fn values() -> Vec<Suit> {
        vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Symbol {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Knight,
    Queen,
    King,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Symbol::Ace => write!(f, "as"),
            &Symbol::Two => write!(f, "2"),
            &Symbol::Three => write!(f, "3"),
            &Symbol::Four => write!(f, "4"),
            &Symbol::Five => write!(f, "5"),
            &Symbol::Six => write!(f, "6"),
            &Symbol::Seven => write!(f, "7"),
            &Symbol::Eight => write!(f, "8"),
            &Symbol::Nine => write!(f, "9"),
            &Symbol::Ten => write!(f, "10"),
            &Symbol::Jack => write!(f, "valet"),
            &Symbol::Knight => write!(f, "cavalier"),
            &Symbol::Queen => write!(f, "dame"),
            &Symbol::King => write!(f, "roi"),
        }
    }
}

impl Symbol {
    pub fn values() -> Vec<Symbol> {
        vec![Symbol::Ace,
             Symbol::Two,
             Symbol::Three,
             Symbol::Four,
             Symbol::Five,
             Symbol::Six,
             Symbol::Seven,
             Symbol::Eight,
             Symbol::Nine,
             Symbol::Ten,
             Symbol::Jack,
             Symbol::Knight,
             Symbol::Queen,
             Symbol::King]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct Face {
    pub suit: Suit,
    pub symbol: Symbol,
}

impl Ord for Face {
    fn cmp(&self, face: &Face) -> Ordering {
        match self.symbol.cmp(&face.symbol) {
            Ordering::Equal => self.suit.cmp(&face.suit),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Trump {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
}

impl fmt::Display for Trump {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Trump::One => write!(f, "1"),
            &Trump::Two => write!(f, "2"),
            &Trump::Three => write!(f, "3"),
            &Trump::Four => write!(f, "4"),
            &Trump::Five => write!(f, "5"),
            &Trump::Six => write!(f, "6"),
            &Trump::Seven => write!(f, "7"),
            &Trump::Eight => write!(f, "8"),
            &Trump::Nine => write!(f, "9"),
            &Trump::Ten => write!(f, "10"),
            &Trump::Eleven => write!(f, "11"),
            &Trump::Twelve => write!(f, "12"),
            &Trump::Thirteen => write!(f, "13"),
            &Trump::Fourteen => write!(f, "14"),
            &Trump::Fifteen => write!(f, "15"),
            &Trump::Sixteen => write!(f, "16"),
            &Trump::Seventeen => write!(f, "17"),
            &Trump::Eighteen => write!(f, "18"),
            &Trump::Nineteen => write!(f, "19"),
            &Trump::Twenty => write!(f, "20"),
            &Trump::TwentyOne => write!(f, "21"),
        }
    }
}

impl Trump {
    pub fn values() -> Vec<Trump> {
        vec![Trump::One,
             Trump::Two,
             Trump::Three,
             Trump::Four,
             Trump::Five,
             Trump::Six,
             Trump::Seven,
             Trump::Eight,
             Trump::Nine,
             Trump::Ten,
             Trump::Eleven,
             Trump::Twelve,
             Trump::Thirteen,
             Trump::Fourteen,
             Trump::Fifteen,
             Trump::Sixteen,
             Trump::Seventeen,
             Trump::Eighteen,
             Trump::Nineteen,
             Trump::Twenty,
             Trump::TwentyOne]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Card {
    Face(Face),
    Trump(Trump),
    Fool,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Card::Fool => write!(f, "excuse"),
            &Card::Face(Face { ref suit, ref symbol }) => write!(f, "{} de {}", symbol, suit),
            &Card::Trump(Trump::One) => write!(f, "petit"),
            &Card::Trump(ref trump) => write!(f, "{} d'atout", trump),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, card: &Card) -> Ordering {
        match (self, card) {
            (&Card::Fool, &Card::Fool) => Ordering::Equal,
            (&Card::Fool, _) => Ordering::Less,
            (_, &Card::Fool) => Ordering::Greater,
            (&Card::Trump(_), &Card::Face(_)) => Ordering::Greater,
            (&Card::Face(_), &Card::Trump(_)) => Ordering::Less,
            (&Card::Trump(trump1), &Card::Trump(trump2)) => trump1.cmp(&trump2),
            (&Card::Face(face1), &Card::Face(face2)) => face1.cmp(&face2),
        }
    }
}
