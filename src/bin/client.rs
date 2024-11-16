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
use std::{collections::HashMap, io::{self, Write}};
use rand::{seq::SliceRandom, thread_rng};
use std::fmt::{self, Display};
use std::cell::RefCell;
use std::rc::Rc;

struct State {
    clients: HashMap<String,Rc<RefCell<Client>>>,
    deck: Deck
}

impl State {
    pub fn new() -> Self {
        State {
            deck: generate_deck().shuffle(),
            clients: HashMap::new(),
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
            Suit::Hearts => s.push('\u{2665}'),
            Suit::Clubs => s.push('\u{2663}'),
            Suit::Spades => s.push('\u{2660}'),
            Suit::Diamonds => s.push('\u{2666}'),
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
        self.clone()
    }
    #[allow(dead_code)]
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

fn get_user_input(prompt: String) -> Result<String, io::Error> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    stdin.read_line(&mut user_input)?;
    user_input.pop(); // get rid of last "\n"
    println!("Got: {}", user_input);
    Ok(user_input)
}

struct Round<'a> {
    poll: f32,
    ante: f32,
    bet: f32,
    table: [Option<Card>; 5],
    clients: Vec<String>,
    state: & 'a mut State,
}

impl<'a> Round<'a> {
    fn new(state: & 'a mut State, clients: Vec<String>) -> Self {
        // state.deck.print();
        Round {
            poll: 0.0,
            ante: 2.00,
            bet: 10.0,
            table: [ None, None, None, None, None ],
            clients,
            state,
        }
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
    fn collect_ante(&mut self) {
        let mut ids_to_remove: Vec<String> = vec![];
        for id in self.clients.clone() {
            // c should be a &mut Rc<RefCell<Client>>
            if let Some(c) = self.state.clients.get_mut(&id.clone()) {
                let mut client = c.borrow_mut();
                if client.cash < 5.0 {
                    println!("User {} ain't got enough cash $$$. Faggot.", id);
                    ids_to_remove.push(client.id.clone());
                }
                client.cash -= self.ante;
                self.poll += self.ante;
            } else {
                println!("Couldn't get user with id: {}", id);
                ids_to_remove.push(id);
            }
        }
        // Remove clients by ID
        for id in ids_to_remove {
            println!("Removing user: {}", id);
            self.clients.retain(|client_id| client_id != &id); // Remove by matching ID
        }
    }
    fn small_blind(&mut self) {
        // makes small bet
    }
    fn big_blind(&mut self) {
        // makes minimum bet
        self.bet = 10.0;
    }
    fn deal_hole_cards(&mut self) {
        let mut ids_to_remove: Vec<String> = vec![];
        for _ in 0..2 {
            for id in self.clients.clone() {
                // c should be a &mut Rc<RefCell<Client>>
                if let Some(c) = self.state.clients.get_mut(&id.clone()) {
                    let mut client = c.borrow_mut();
                    if let Some(card) = self.state.deck.get_card() {
                        client.hand.push(card);
                    } else {
                        panic!("Out of cards");
                    }
                } else {
                    println!("Couldn't get user with id: {}", id);
                    ids_to_remove.push(id);
                }
            }
        }
        // Remove clients by ID
        for id in ids_to_remove {
            println!("Removing user: {}", id);
            self.clients.retain(|client_id| client_id != &id); // Remove by matching ID
        }
        for id in self.clients.clone() {
            if let Some(c) = self.state.clients.get_mut(&id.clone()) {
                println!("{}", c.borrow());
            }
        }
    }
    // and deal hole cards
    fn pre_flop_bet(&mut self) {
        // deal hole cards - two cards per player, one at a time, clockwise

    }
    fn flop(&mut self) {
        // burns one card - to prevent card tracking 
        // deals three comunity cards
        self.table[0] = self.state.deck.get_card();
        self.table[1] = self.state.deck.get_card();
        self.table[2] = self.state.deck.get_card();
        print!("flop:\t\t");
        self.show_table();
    }
    fn bet(&mut self) {
        let mut ids_to_remove: Vec<String> = vec![];
        for id in self.clients.clone() {
            if let Some(c) = self.state.clients.get(&id.clone()){
                let mut client = c.borrow_mut();
                match get_user_input(format!("(user:{})Enter your bet: ", id).to_string()) {
                    Ok(input) => {
                        // rewritre
                        let bet: f32 = input.trim().parse().expect("failed to unwrap variable");
                        client.cash -= bet;
                        self.poll += bet;
                    },
                    // if this fails then something's wrong
                    Err(err) => panic!("Error in getting user input: {}", err),
                }

            } else {
                println!("Couldn't get user with id: {}", id);
                ids_to_remove.push(id);
            }
        }
        // Remove clients by ID
        for id in ids_to_remove {
            println!("Removing user: {}", id);
            self.clients.retain(|client_id| client_id != &id); // Remove by matching ID
        }
    }
    fn turn(&mut self) {
        // burns one card - to prevent card tracking 
        // deals fourth comunity cards
        self.table[3] = self.state.deck.get_card();
        print!("turn:\t\t");
        self.show_table();
    }
    fn river(&mut self) {
        // burns one card - to prevent card tracking 
        // deals fift and last comunity cards
        self.table[4] = self.state.deck.get_card();
        print!("river:\t\t");
        self.show_table();
    }
    fn showdown(&mut self) {
    }
}

#[derive(Clone)]
struct Client {
    pub id: String,
    pub hand: Vec<Card>,
    pub cash: f32,
}

impl Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for c in self.hand.clone() {
            s.push_str(&format!("[{}]", c).to_string());
        }
        write!(f, "id: {}\tcash: {}\thand: {}", self.id,self.cash,s)
    }
}

impl Client {
    fn new(id: String) -> Self {
        Client {
            id,
            hand: vec![],
            cash: 100.0,
        }
    }
    fn reset(&mut self) {
        self.hand = vec![];
    }
}

fn poker() -> Result<(), io::Error> {
    print!("Enter number of users: ");
    let _ = io::stdout().flush();
    let mut user_input: String = String::new();
    io::stdin().read_line(& mut user_input)?;
    let users: u16 = user_input.trim().parse().expect("Failed to parse user input");
    println!("Making {} users...", users);

    let mut state = State::new();
    let mut clients_on: HashMap<String, Rc<RefCell<Client>>> = HashMap::new();

    for i in 0..users {
        let client: Rc<RefCell<Client>> = Rc::new(RefCell::new(Client::new(i.to_string())));
        let client_ref = client.borrow();
        // println!("Client ID: {}", client_ref.id);
        clients_on.insert(client_ref.id.clone(), Rc::clone(&client));
    }
    state.clients = clients_on.to_owned();

    loop {
        let client_ids: Vec<String> = clients_on
            .iter()
            .map(|(_, c)| {
                let client = c.borrow();
                client.id.clone()
            })
            .collect::<Vec<String>>();
        
        let mut r = Round::new(&mut state, client_ids);
        r.collect_ante();
        r.small_blind();
        r.big_blind();
        r.deal_hole_cards();
        // big blind can raise, too
        r.pre_flop_bet(); // and collect
        r.flop(); // three cards
        r.bet(); // fold, call, raise, check
        r.turn(); // 4th card
        r.bet();
        r.river();
        r.bet();
        r.showdown();

        for (_,c) in state.clients.iter() {
            c.borrow_mut().reset();
        }

        // break with 'q'
        let i = get_user_input("Quit? [q/nil]: ".to_string())?;
        if i == String::from("q") {
            return Ok(());
        }
    }
    // Ok(())
}

fn main() {
    let _ = poker();
}
