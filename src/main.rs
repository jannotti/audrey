use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Suit {
    Clubs = 0b00_00_0000,
    Diamonds = 0b01_00_0000,
    Hearts = 0b10_00_0000,
    Spades = 0b11_00_0000,
}
impl From<u8> for Suit {
    fn from(val: u8) -> Self {
        match val {
            0b00_000000 => Suit::Clubs,
            0b01_000000 => Suit::Diamonds,
            0b10_000000 => Suit::Hearts,
            0b11_000000 => Suit::Spades,
            _ => panic!(val),
        }
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "c"),
            Suit::Diamonds => write!(f, "d"),
            Suit::Hearts => write!(f, "h"),
            Suit::Spades => write!(f, "s"),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl From<u8> for Rank {
    fn from(val: u8) -> Self {
        match val {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => panic!(val),
        }
    }
}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = *self as u8;
        if val < 10 {
            return write!(f, "{}", val);
        }
        match self {
            Rank::Ten => write!(f, "T"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
            _ => write!(f, "?"),
        }
    }
}

#[derive(Debug)]
struct Card(u8);

impl Card {
    fn new(rank: impl Into<Rank>, suit: Suit) -> Card {
        Card((suit as u8) | (rank.into() as u8))
    }
    fn rank(&self) -> Rank {
        Rank::from(self.0 & 0b1111)
    }
    fn suit(&self) -> Suit {
        Suit::from(self.0 & 0b11000000)
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}

fn main() {
    let card = Card::new(6, Suit::Hearts);
    println!("{:?}", card.suit());
    println!("{}", card);
}
