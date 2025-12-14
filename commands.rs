pub enum Command {
    NewTag,
    Exit,
    Help,
    Clear,
    Unknown,
}

pub fn parse(input: &str) -> Command {
    match input.trim().to_lowercase().as_str() {
        "new tag" => Command::NewTag,
        "exit" | "e" => Command::Exit,
        "help" | "h" => Command::Help,
        "clear" => Command::Clear,
        _ => Command::Unknown,
    }
}
