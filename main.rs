use std::io::{self, Write};

mod commands;
mod tag;

use commands::{parse, Command};
use tag::new_tag_flow;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();

        match parse(cmd) {
            Command::NewTag => new_tag_flow(),
            Command::Exit => break,
            Command::Help => println!("Welcome to Yuno FAQ Manager\nPossible commands:\n\nnew tag - create new tag\nexit - terminate this programm\n"),
            Command::Unknown => println!("unknown command"),
        }
    }
}
