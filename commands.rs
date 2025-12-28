pub enum Command {
    NewTag,
    NewThema,
    NewQa,
    Exit,
    Help,
    Clear,
    Unknown,
}

pub fn parse(input: &str) -> Command {
    match input.trim().to_lowercase().as_str() {
        "new tag" => Command::NewTag,
        "new thema" => Command::NewThema,
        "new qa" => Command::NewQa,
        "exit" | "e" => Command::Exit,
        "help" | "h" => Command::Help,
        "clear" | "c"=> Command::Clear,
        _ => Command::Unknown,
    }
}
