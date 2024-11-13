use four_t::common::state::State;
/*
use crossterm::{
    cursor::{Hide, MoveTo},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{size, Clear, ClearType, SetSize},
    ExecutableCommand,
};

use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
*/
use std::io::{self, Write};
use rand::{seq::SliceRandom, thread_rng};

struct ClientState {
    state: State,
}

impl ClientState {
    fn new() -> Self {
        ClientState {
            state: State::new(),
        }
    }
}

#[derive(Debug,Clone, Copy)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}


#[derive(Debug,Clone, Copy)]
enum Value {
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

#[derive(Debug,Clone, Copy)]
struct Card {
    value: Value,
    suit: Suit
}

#[derive(Debug,Clone)]
struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Self {
        Deck {
            cards: vec![],
        }
    }
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
    fn print(&self) {
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
    fn get_card(&mut self) -> Option<Card> {
        if let Some(c) = self.cards.pop() {
            return Some(c);
        }
        None
    }
}

fn generate_deck() -> Deck {
    let suits: Vec<Suit> = vec![Suit::Hearts, Suit::Spades, Suit::Clubs, Suit::Diamonds];
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

fn get_user_input() -> Result<String, io::Error> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    print!("Enter your input: ");
    let _ = std::io::stdout().flush();
    stdin.read_line(&mut user_input)?;
    user_input.pop(); // get rid of last "\n"
    println!("Got: {}", user_input);
    Ok(user_input)
}

fn black_jack() -> Result<(), io::Error> {
    let state = State::new();
    let _client_state = ClientState::new();

    let mut deck = generate_deck();
    println!("{}", deck.cards.len());
    deck.shuffle();
    deck.print();
    'mainloop: loop {
        if let Some(c) = deck.get_card() {
            println!("{:?}", c);
        }
        let input = get_user_input()?;
        if input == String::from("quit") {
            break 'mainloop;
        }
    }
    Ok(())
}


fn main() {
    let _ = black_jack();
}
