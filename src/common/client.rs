use std::fmt::{self, Display};

use crate::common::cards::Card;

#[derive(Clone)]
pub struct Client {
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
    pub fn new(id: String) -> Self {
        Client {
            id,
            hand: vec![],
            cash: 100.0,
        }
    }
    pub fn reset(&mut self) {
        self.hand = vec![];
    }
}

