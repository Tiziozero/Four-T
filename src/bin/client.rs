use four_t::common::state::State;
use four_t::common::cards::Card;
use four_t::common::client::Client;
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
use std::cell::RefCell;
use std::rc::Rc;


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
    dealer: String,
    state: & 'a mut State,
}

impl<'a> Round<'a> {
    fn new(state: & 'a mut State, clients: Vec<String>, dealer: String) -> Self {
        // state.deck.print();
        Round {
            poll: 0.0,
            ante: 2.00,
            bet: 10.0,
            table: [ None, None, None, None, None ],
            clients,
            dealer,
            state,
        }
    }
    fn show_users(&mut self) {
        for id in self.clients.clone() {
            if let Some(c) = self.state.clients.get_mut(&id.clone()) {
                println!("{}", c.borrow());
            }
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
        let dealer_id = self.dealer.clone();
        let client_ids = self.clients.clone();

        if let Some(dealer_pos) = client_ids.iter().position(|id| *id == dealer_id) {
            let small_blind_index = (dealer_pos + 1) % client_ids.len();
            let small_blind_client_id = client_ids[small_blind_index].clone();
            if let Some(c) = self.state.clients.get(&small_blind_client_id.clone()) {
                let mut client = c.borrow_mut();
                client.cash -= self.bet / 2.0;
                println!("got small blind from: {}", client.id.clone());
            } else {
                panic!("Missing user {}", small_blind_client_id);
            }
        }
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
        // add "check"
        let mut ids_to_remove: Vec<String> = vec![];
        for id in self.clients.clone() {
            if let Some(c) = self.state.clients.get(&id.clone()){

                let prompt = format!("Current bet is: {}; fold(f), call(c) or rasie(r): ", self.bet).to_string();
                // if this fails the something's wrong
                let user_input = get_user_input(prompt).expect("failed to get user input");// fold, call, raise
                //
                match user_input.as_str() {
                    "f" => {
                        ids_to_remove.push(id.clone());
                        println!("User {} has folded", id.clone());
                    },
                    "c" => {
                        let mut client = c.borrow_mut();
                        client.cash -= self.bet;
                        self.poll += self.bet;
                    }
                    "r" => {
                        // raise
                        let user_input = get_user_input(
                            format!("Current bet: {} ", self.bet)
                                .to_string()
                            ).expect("failed to get user input");
                        let new_bet: f32 = user_input
                            .trim().parse().expect("failed to parse message");
                        if new_bet < self.bet * 2.0 {
                            panic!("new bet needs to be at leas twice current bet: current bet: {}, new bet: {}", self.bet, new_bet);
                        }
                    }
                    _ => {
                        // invalid operation
                        panic!("\"{}\"; invalid operation", user_input);
                    }
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
fn poker() -> Result<(), io::Error> {
    print!("Enter number of users: ");
    let _ = io::stdout().flush();
    let mut user_input: String = String::new();
    io::stdin().read_line(& mut user_input)?;
    let users: u16 = user_input.trim().parse().expect("Failed to parse user input");
    // if users < 2 || users > 10 {
    //     panic!("Users must be between 2 and 10");
    // }
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


    let mut dealer_ids = clients_on.keys().clone();

    loop {
        let mut dealer: String;
        'get_dealer_loop: loop {
            // get user id for dealer, this is going to be
            // the next user in the keys of the vector ov clients
            'get_dealer_id: loop {
                // if there is a next in the ids set dealer to it
                if let Some(dealer_id) = dealer_ids.next() {
                    dealer = dealer_id.clone();
                    break 'get_dealer_id;
                // else reset ids with new iter
                } else {
                    dealer_ids = clients_on.keys().clone();
                }
            }


            // if user still in vector break loo[ 
            let client_rc = state.clients.get(&dealer.clone());
            if let Some(_client) = client_rc {
                break 'get_dealer_loop;
            }
            println!("Couldn't find a dealer")
        }

        let client_ids: Vec<String> = clients_on
            .iter()
            .map(|(_, c)| {
                let client = c.borrow();
                client.id.clone()
            })
            .collect::<Vec<String>>();
        
        let mut r = Round::new(&mut state, client_ids, dealer.clone());
        r.show_users();
        r.collect_ante();
        r.show_users();
        r.small_blind();
        r.show_users();
        r.big_blind();
        r.show_users();
        r.deal_hole_cards();
        r.show_users();
        // big blind can raise, too
        r.pre_flop_bet(); // and collect
        r.show_users();
        r.flop(); // three cards
        r.show_users();
        r.bet(); // fold, call, raise, check
        r.show_users();
        r.turn(); // 4th card
        r.show_users();
        r.bet();
        r.show_users();
        r.river();
        r.show_users();
        r.bet();
        r.show_users();
        r.showdown();
        r.show_users();

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
