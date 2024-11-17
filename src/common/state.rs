use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::cards::{Deck, generate_deck};
use crate::common::client::Client;

pub struct State {
    pub clients: HashMap<String,Rc<RefCell<Client>>>,
    pub deck: Deck
}

impl State {
    pub fn new() -> Self {
        State {
            deck: generate_deck().shuffle(),
            clients: HashMap::new(),
        }
    }
}
