use anyhow::{anyhow, Result};
use std::fmt;
use std::str::FromStr;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    Clubs = 0b00_00_0000,
    Diamonds = 0b01_00_0000,
    Hearts = 0b10_00_0000,
    Spades = 0b11_00_0000,
}
impl Suit {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Clubs, Self::Diamonds, Self::Hearts, Self::Spades]
            .iter()
            .copied()
    }
}
impl FromStr for Suit {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let ch = s.chars().nth(0).ok_or(anyhow!("empty string"))?;
        match ch {
            'c' | 'C' => Ok(Self::Clubs),
            'd' | 'D' => Ok(Self::Diamonds),
            'h' | 'H' => Ok(Self::Hearts),
            's' | 'S' => Ok(Self::Spades),
            _ => Err(anyhow!("Weird suit {}", ch)),
        }
    }
}
impl From<char> for Suit {
    fn from(val: char) -> Self {
        Self::from_str(&val.to_string()).unwrap()
    }
}
impl From<u8> for Suit {
    fn from(val: u8) -> Self {
        match val {
            0b00_000000 => Self::Clubs,
            0b01_000000 => Self::Diamonds,
            0b10_000000 => Self::Hearts,
            0b11_000000 => Self::Spades,
            _ => panic!(val),
        }
    }
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Clubs => write!(f, "c"),
            Self::Diamonds => write!(f, "d"),
            Self::Hearts => write!(f, "h"),
            Self::Spades => write!(f, "s"),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Rank {
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
impl Rank {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
            Self::Jack,
            Self::Queen,
            Self::King,
            Self::Ace,
        ]
        .iter()
        .copied()
    }
}
impl FromStr for Rank {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let ch = s.chars().nth(0).ok_or(anyhow!("empty str"))?;
        match ch {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(anyhow!("Weird rank {}", ch)),
        }
    }
}
impl From<char> for Rank {
    fn from(val: char) -> Self {
        Self::from_str(&val.to_string()).unwrap()
    }
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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card(u8);

impl Card {
    pub fn new(rank: impl Into<Rank>, suit: impl Into<Suit>) -> Card {
        Self((suit.into() as u8) | (rank.into() as u8))
    }
    pub fn rank(&self) -> Rank {
        Rank::from(self.0 & 0b1111)
    }
    pub fn suit(&self) -> Suit {
        Suit::from(self.0 & 0b11_00_0000)
    }
}
impl FromStr for Card {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s.chars().nth(0).ok_or(anyhow!("empty str"))?.to_string();
        let s = s.chars().nth(1).ok_or(anyhow!("short str"))?.to_string();

        Ok(Card::new(Rank::from_str(&r)?, Suit::from_str(&s)?))
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}
