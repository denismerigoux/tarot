use std::fmt;

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

pub struct Face {
    pub suit: Suit,
    pub symbol: Symbol,
}

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

pub enum Card {
    Face(Face),
    Trump(Trump),
    Fool,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Card::Fool => write!(f, "Excuse"),
            &Card::Face(Face { ref suit, ref symbol }) => write!(f, "{} de {}", symbol, suit),
            &Card::Trump(Trump::One) => write!(f, "petit"),
            &Card::Trump(ref trump) => write!(f, "{} d'atout", trump),
        }
    }
}
