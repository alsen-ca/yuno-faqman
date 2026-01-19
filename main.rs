use std::io::{self, Write};
use crossterm::{
    cursor::{MoveToColumn, MoveTo},
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand
};


mod commands;
mod ui;
mod controller;
mod history;
mod domain;

use commands::{parse, Command};
use ui::tag::new_tag_flow;
use ui::thema::new_thema_flow;
use ui::qa::new_qa_flow;
use controller::tag::{handle_new_tag, handle_get_tag, fetch_and_store_tags};
use controller::thema::{handle_new_thema, handle_get_thema, fetch_and_store_themas};
use controller::qa::{handle_new_qa, handle_get_qa};
use history::History;
use domain::thema::{get_themas};
use domain::tag::{get_tags};

enum ReplAction {
    Continue,
    Exit,
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    print_greeting();
    print_help();
    fetch_data();
    let _ = disable_raw_mode();
    for thema in get_themas() {
        println!("THEMEN loaded from memory! ID: {}, Title: {}", thema.id, thema.title)
    }
    for tag in get_tags() {
        println!("TAGS loaded from memory! ID: {}, ENglish: {}, German: {}, Spanish: {}", tag.id, tag.en_og, tag.de_trans, tag.es_trans)
    }
    let _ = enable_raw_mode();


    let mut history = History::load(".history").unwrap();
    let mut buffer = String::new();

    loop {
        // Render prompt
        print!("> ");
        print!("{}", buffer);
        stdout.flush().unwrap();

        match read_keys(&mut buffer, &mut history)? {
            ReplAction::Continue => {}
            ReplAction::Exit => break,
        }

        // Clear current line and re-render
        stdout.execute(MoveToColumn(0))?;
        stdout.execute(Clear(ClearType::CurrentLine))?;
    }

    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn read_keys(buffer: &mut String, history: &mut History) -> io::Result<ReplAction> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return Ok(ReplAction::Exit);
            }
            
            KeyCode::Char(c) => { buffer.push(c); }

            KeyCode::Backspace => { buffer.pop(); }

            KeyCode::Up => {
                if let Some(cmd) = history.previous() {
                    buffer.clear();
                    buffer.push_str(cmd);
                }
            }

            KeyCode::Down => {
                if let Some(cmd) = history.next() {
                    buffer.clear();
                    buffer.push_str(cmd);
                }
            }

            KeyCode::Enter => {
                println!();

                let cmd = buffer.trim().to_string();
                buffer.clear();
                history.reset_cursor();

                if !cmd.is_empty() {
                    history.push(&cmd).ok();
                }

                match parse(&cmd) {
                    Command::NewTag => {
                        disable_raw_mode()?;
                        if let Some(tag) = new_tag_flow() {
                            tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(handle_new_tag(tag));
                        }
                        enable_raw_mode()?;
                    }
                    Command::GetTag { lang, question} => {
                        disable_raw_mode()?;
                        tokio::runtime::Runtime::new().unwrap().block_on(handle_get_tag(lang, question));
                        enable_raw_mode()?;
                    }
                    Command::NewThema => {
                        disable_raw_mode()?;
                        if let Some(thema) = new_thema_flow() {
                            tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(handle_new_thema(thema));
                        }
                        enable_raw_mode()?;
                    }
                    Command::GetThema(to_search) => {
                        disable_raw_mode()?;
                        tokio::runtime::Runtime::new().unwrap().block_on(handle_get_thema(to_search));
                        enable_raw_mode()?;
                    }
                    Command::NewQa => {
                        disable_raw_mode()?;
                        
                        if let Some(qa) = new_qa_flow() {
                            tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(handle_new_qa(qa));
                        }
                        enable_raw_mode()?;
                    }
                    Command::GetQa(to_search) => {
                        disable_raw_mode()?;
                        tokio::runtime::Runtime::new().unwrap().block_on(handle_get_qa(to_search));
                        enable_raw_mode()?;
                    }
                    Command::Help => print_help(),
                    Command::Clear => clear_screen(),
                    Command::Exit => return Ok(ReplAction::Exit),
                    Command::Unknown => println!("unknown command"),
                }
            }
            _ => {}
        }
    }
    Ok(ReplAction::Continue)
}

fn clear_screen() {
    let mut out = io::stdout();
    out.execute(Clear(ClearType::All)).unwrap();
    out.execute(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
}

fn print_help() {
    let _ = disable_raw_mode();
    println!();
    println!("Possible commands:\n");

    println!("new tag - create new tag");
    println!("new thema - create new thema");
    println!("new qa - create new combination of question and answer");
    println!("get thema [<thema title> | all] - find thema by title or all");
    println!("get tag [<tag en_og | de_trans | es_trans> | all] - find tag by any language or all");
    println!("get qa [<qa question> | all] - find qa by exact question or all");
    
    println!("\nhelp / h - print this help guide");
    println!("clear / c - clear the screen");
    println!("exit / e - terminate this programm\n");
    let _ = enable_raw_mode();
}
fn print_greeting() {
    let _ = disable_raw_mode();
    println!("Welcome to Yuno FAQ Manager");
    let _ = enable_raw_mode();
}

fn fetch_data() {
    if let Err(e) = tokio::runtime::Runtime::new().unwrap().block_on(fetch_and_store_themas()) {
        eprintln!("Failed to fetch and store themas: {}", e);
    }
    if let Err(e) = tokio::runtime::Runtime::new().unwrap().block_on(fetch_and_store_tags()) {
        eprintln!("Failed to fetch and store tags: {}", e);
    }
}