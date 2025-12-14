use std::io::{stdout, Write};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
    cursor::MoveTo,
    ExecutableCommand
};

pub fn new_tag_flow() {
    let mut fields = vec![
        ("en_og", String::new()),
        ("de_trans", String::new()),
        ("es_trans", String::new()),
        ("command", String::new()),
    ];

    let mut cursor: usize = 0;
    terminal::enable_raw_mode().unwrap();

    loop {
        print_form(&fields, cursor);

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Up => {
                    if cursor > 0 {
                        cursor -= 1;
                    }
                }
                KeyCode::Down => {
                    if cursor + 1 < fields.len() {
                        cursor += 1;
                    }
                }
                KeyCode::Char(c) => {
                    fields[cursor].1.push(c);
                }
                KeyCode::Backspace => {
                    fields[cursor].1.pop();
                }
                KeyCode::Enter => {
                    if fields[cursor].0 == "command" {
                        match fields[cursor].1.as_str() {
                            "save" => {
                                terminal::disable_raw_mode().unwrap();
                                println!("--- TAG CREATED ---");
                                for (label, value) in fields.iter().take(3) {
                                    println!("{}: {}", label, value);
                                }
                                break;
                            }
                            "exit" => {
                                terminal::disable_raw_mode().unwrap();
                                println!("\ntag creation cancelled");
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                KeyCode::Esc => {
                    terminal::disable_raw_mode().unwrap();
                    println!("\ntag creation cancelled");
                    break;
                }
                _ => {}
            }
        }
    }
}

fn print_form(fields: &Vec<(&str, String)>, cursor: usize) {
    let mut out = stdout();

    // Wipes the screen
    out.execute(Clear(ClearType::All)).unwrap();
    // Forces cursor to top-left corner
    out.execute(MoveTo(0, 0)).unwrap();

    for (i, (label, value)) in fields.iter().enumerate() {
        if i == cursor {
            writeln!(out, "\r> {}: {}", label, value).unwrap();
        } else {
            writeln!(out, "\r  {}: {}", label, value).unwrap();
        }
    }
    writeln!(out, "\rUse ↑ ↓ to move | type to edit | save / exit | Esc to quit").unwrap();
    writeln!(out, "\r").unwrap();

    out.flush().unwrap();
}
