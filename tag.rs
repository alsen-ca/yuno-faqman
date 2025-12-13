use std::io::{self, Write};

pub fn new_tag_flow() {
    let mut fields = vec![
        ("en_og", String::new()),
        ("de_trans", String::new()),
        ("es_trans", String::new()),
    ];

    let mut cursor: usize = 0;

    loop {
        print_form(&fields, cursor);

        let input = read_line("command");

        match input.as_str() {
            "next" => {
                if cursor + 1 < fields.len() {
                    cursor += 1;
                }
            }
            "prev" => {
                if cursor > 0 {
                    cursor -= 1;
                }
            }
            "save" => {
                println!("--- TAG CREATED ---");
                for (label, value) in &fields {
                    println!("{}: {}", label, value);
                }
                println!();
                break;
            }
            "exit" => {
                println!("tag creation cancelled");
                println!();
                break;
            }
            _ => {
                fields[cursor].1 = input;
            }
        }
    }
}

fn print_form(fields: &Vec<(&str, String)>, cursor: usize) {
    println!();
    for (i, (label, value)) in fields.iter().enumerate() {
        if i == cursor {
            println!("> {}: {}", label, value);
        } else {
            println!("  {}: {}", label, value);
        }
    }
    println!("commands: next | prev | save | exit");
}

fn read_line(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
