use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "({}, {})", self.0, self.1)?;
        write!(f, "({}, {})", self.2, self.3)
    }
}

#[derive(Debug)]
struct Card {
    rank: u8,
    suit: u8,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.rank)?;
        match self.suit {
            0 => write!(f, "c"),
            1 => write!(f, "d"),
            2 => write!(f, "h"),
            3 => write!(f, "s"),
            _ => panic!("ack"),
        }
    }
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    let card = Card { rank: 1, suit: 2 };
    println!("{:?}", card);
    println!("{}", card);
}
