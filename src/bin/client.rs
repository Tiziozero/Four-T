// use four_t::common::state::State;
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
use std::fmt::{self, Display};

struct State {
    clients: Vec<Client>,
    deck: Deck
}

impl State {
    pub fn new() -> Self {
        State {
            deck: generate_deck().shuffle(),
            clients: vec![],
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
impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        match self {
            Suit::Hearts => s.push('H'),
            Suit::Clubs => s.push('C'),
            Suit::Spades => s.push('S'),
            Suit::Diamonds => s.push('D'),
        }
        write!(f, "{}", s)
    }
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
struct Card {
    value: Value,
    suit: Suit
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
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
    fn shuffle(&mut self) -> Self {
        self.cards.shuffle(&mut thread_rng());
        println!("{}", self.cards[0]);
        self.clone()
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

struct Round {
    ante: f32,
    table: [Option<Card>; 5],
    clients: Vec<Client>,
}

impl Round {
    fn new(state: &mut State, clients: Vec<Client>) -> Self {
        state.deck.print();
        Round {
            ante: 5.00,
            table: [ state.deck.get_card(),
                state.deck.get_card(), None, None, None ],
            clients: clients,
        }
    }
    fn flop(&mut self, state: &mut State) {
        self.table[2] = state.deck.get_card();
    }
    fn show_table(&self) {
        for c in self.table {
            if let Some(card) = c {
                print!("[{}]", card);
            } else {
                print!("[__]");
            }
        }
        print!("\n");
    }
}

#[derive(Clone)]
struct Client {
    id: String,
    hand: Vec<Card>,
}

impl Client {
    fn new(id: String) -> Self {
        Client {
            id: id,
            hand: vec![],
        }
    }
    fn reset(&mut self) {
        self.hand = vec![];
    }
}

fn black_jack() -> Result<(), io::Error> {
    print!("Enter number of users: ");
    let _ = io::stdout().flush();
    let mut user_input: String = String::new();
    io::stdin().read_line(& mut user_input)?;
    let users: u16 = user_input.trim().parse().expect("Failed to parse user input");
    println!("Making {} users...", users);

    let mut state = State::new();
    let mut clients_on: Vec<Client> = vec![];
    
    for i in 0..users {
        let c = Client::new(format!("{}", i));
        clients_on.push(c);
    }

    'mainloop: loop {
        let r = Round::new(&mut state, clients_on.clone());
        r.show_table();
        break
    }
    Ok(())
}

fn main() {
    let _ = black_jack();
}
