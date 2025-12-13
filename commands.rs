pub enum Command {
    NewTag,
    Exit,
    Help,
    Unknown,
}

pub fn parse(input: &str) -> Command {
    match input.trim() {
        "new tag" => Command::NewTag,
        "exit" => Command::Exit,
        "help" => Command::Help,
        _ => Command::Unknown,
    }
}
