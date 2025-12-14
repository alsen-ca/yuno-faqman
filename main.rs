use std::io::{self, stdout, Write};
use crossterm::{
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    ExecutableCommand
};

mod commands;
mod ui;
mod controller;

use commands::{parse, Command};
use ui::tag::new_tag_flow;
use controller::tag::handle_new_tag;

fn main() {
    print_help();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();

        match parse(cmd) {
            Command::NewTag => {
                if let Some(tag) = new_tag_flow() {
                    handle_new_tag(tag);
                }
            }
            Command::Exit => break,
            Command::Help => print_help(),
            Command::Clear => clear_screen(),
            Command::Unknown => println!("unknown command"),
        }
    }
}

fn clear_screen() {
    let mut out = stdout();
    out.execute(Clear(ClearType::All)).unwrap();
    out.execute(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
}

fn print_help() {
    println!("Welcome to Yuno FAQ Manager");
    println!("Possible commands:\n");

    println!("new tag - create new tag");
    println!("new thema - create new thema");
    println!("new qa - create new combination of question and answer");
    
    println!("\nhelp / h - print this help guide");
    println!("exit / e - terminate this programm\n");
}
