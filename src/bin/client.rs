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
    println!("Hello from client: {}", state.number);
    'mainloop: loop {
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
