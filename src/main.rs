use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::str::FromStr;

mod cards;
use crate::cards::{Card, Rank, Suit};

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<Card>,
}
impl Hand {
    fn add(&mut self, card: Card) {
        self.cards.push(card);
        self.cards.sort();
        self.cards.reverse();
    }
    fn next_suit(suit: Suit) -> Suit {
        match suit {
            Suit::Clubs => Suit::Spades, // Report error / None instead?
            Suit::Diamonds => Suit::Clubs,
            Suit::Hearts => Suit::Diamonds,
            Suit::Spades => Suit::Hearts,
        }
    }
}
impl Default for Hand {
    fn default() -> Self {
        Self { cards: Vec::new() }
    }
}
impl FromStr for Hand {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut cards: Vec<Card> = Vec::with_capacity(13);
        let mut suit = Suit::Spades;
        for ch in s.chars() {
            if ch == '.' {
                suit = Hand::next_suit(suit);
                continue;
            }
            cards.push(Card::new(Rank::from(ch), suit))
        }
        Ok(Hand { cards })
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut suit = Suit::Spades;
        for card in self.cards.iter() {
            while card.suit() != suit {
                write!(f, ".")?;
                suit = Hand::next_suit(suit);
            }
            write!(f, "{}", card.rank())?
        }
        while suit != Suit::Clubs {
            write!(f, ".")?;
            suit = Hand::next_suit(suit);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(rank, suit))
            }
        }
        Self { cards }
    }
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }
    fn sort(&mut self) {
        self.cards.sort()
    }
    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    fn top(&self) -> Option<&Card> {
        self.cards.last()
    }
    fn deal(&mut self, num_hands: usize, num_cards: usize) -> Result<Vec<Hand>> {
        let mut hands: Vec<Hand> = Vec::with_capacity(num_hands);
        hands.resize_with(num_hands, Default::default);
        for _ in 0..num_cards {
            for h in 0..num_hands {
                hands[h].add(self.draw().ok_or(anyhow!("out of cards"))?)
            }
        }
        return Ok(hands);
    }
}

fn main() -> Result<()> {
    let card = Card::from_str("AH")?;
    println!("{:?}", card.suit());
    println!("{}", card);

    let hand = Hand::from_str("AKT6.52.T98.KJ52")?;
    println!("{}", hand);

    let mut deck = Deck::new();
    deck.shuffle();
    let hands = deck.deal(4, 13)?;
    println!("{} {}", hands[0], hands[1]);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn everything_is_fine() -> Result<()> {
        Ok(())
    }

    #[test]
    fn card_tricks() {
        let two_hearts = Card::new(2, Suit::Hearts);
        assert_eq!(two_hearts.rank(), Rank::Two);
        assert_eq!(two_hearts.suit(), Suit::Hearts);
        assert_eq!(two_hearts, Card::new('2', Suit::Hearts));
        assert_eq!(two_hearts, Card::new('2', 'h'));
        assert_eq!(two_hearts, Card::new('2', 'H'));
        assert_eq!(two_hearts, Card::from_str("2h").expect("no parse"));
    }

    fn assert_hand_io(s: &str) {
        let hand = Hand::from_str(s).expect("no parse");
        assert_eq!(s, hand.to_string())
    }
    #[test]
    fn hand_io() {
        assert_hand_io("AKT6.52.T98.KJ52");
        assert_hand_io("AK6T..T9852.KJ52");
        assert_hand_io("AK6T.T9852.KJ52.");
        assert_hand_io(".AK6T.T9852.KJ52");
    }

    #[test]
    fn swab_the_decks() -> Result<()> {
        let mut deck = Deck::new();
        let top = deck.draw().ok_or(anyhow!("None"))?;
        assert_eq!(Card::new(Rank::Ace, Suit::Spades), top);
        let next = deck.draw().ok_or(anyhow!("None"))?;
        assert_eq!(Card::new(Rank::King, Suit::Spades), next);
        deck.shuffle();
        let maybe = deck.draw().ok_or(anyhow!("None"))?;
        assert_ne!(Card::new(Rank::Queen, Suit::Spades), maybe);
        deck.sort();
        let queen = deck.draw().ok_or(anyhow!("None"))?;
        assert_eq!(Card::new(Rank::Queen, Suit::Spades), queen);
        Ok(())
    }
}
