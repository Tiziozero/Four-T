use rand::{seq::SliceRandom, thread_rng};
use std::fmt::{self, Display};
#[derive(Debug,Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        match self {
            Suit::Hearts => s.push('\u{2665}'),
            Suit::Clubs => s.push('\u{2663}'),
            Suit::Spades => s.push('\u{2660}'),
            Suit::Diamonds => s.push('\u{2666}'),
        }
        write!(f, "{}", s)
    }
}


#[derive(Debug,Clone, Copy)]
pub enum Value {
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
    Queen,
    King,
    Ace
}
impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        match self {
            Value::Two => s.push('2'),
            Value::Three => s.push('3'),
            Value::Four => s.push('4'),
            Value::Five => s.push('5'),
            Value::Six => s.push('6'),
            Value::Seven => s.push('7'),
            Value::Eight => s.push('8'),
            Value::Nine => s.push('9'),
            Value::Ten => s.push('T'),
            Value::Jack => s.push('J'),
            Value::Queen => s.push('Q'),
            Value::King => s.push('K'),
            Value::Ace => s.push('A'),
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug,Clone, Copy)]
pub struct Card {
    pub value: Value,
    pub suit: Suit
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

#[derive(Debug,Clone)]
pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            cards: vec![],
        }
    }
    pub fn shuffle(&mut self) -> Self {
        self.cards.shuffle(&mut thread_rng());
        self.clone()
    }
    #[allow(dead_code)]
    pub fn print(&self) {
        for c in &self.cards {
            let card = *c;
            let mut s = String::new();
            match card.suit {
                Suit::Hearts => s.push('H'),
                Suit::Clubs => s.push('C'),
                Suit::Spades => s.push('S'),
                Suit::Diamonds => s.push('D'),
            }
            match card.value {
                Value::Two => s.push('T'),
                Value::Three => s.push('T'),
                Value::Four => s.push('F'),
                Value::Five => s.push('F'),
                Value::Six => s.push('S'),
                Value::Seven => s.push('S'),
                Value::Eight => s.push('E'),
                Value::Nine => s.push('N'),
                Value::Ten => s.push('T'),
                Value::Jack => s.push('J'),
                Value::Queen => s.push('Q'),
                Value::King => s.push('K'),
                Value::Ace => s.push('A'),
            }
            print!("{};", s)
        }
        println!("");
    }
    pub fn get_card(&mut self) -> Option<Card> {
        if let Some(c) = self.cards.pop() {
            return Some(c);
        }
        None
    }
}
pub fn generate_deck() -> Deck {
    let suits: Vec<Suit> = vec![Suit::Hearts, Suit::Spades,
        Suit::Clubs, Suit::Diamonds];
    let vals: Vec<Value> = vec![
        Value::Two, Value::Three, Value::Four, Value::Five, Value::Six,
        Value::Seven, Value::Eight,  Value::Nine, Value::Ten,
        Value::Jack, Value::Queen, Value::King, Value::Ace
    ];
    let mut d = Deck::new();
    for s in &suits {
        for v in &vals {
            let c = Card{suit: *s, value: *v};
            d.cards.push(c);
        }
    }
    d
}
