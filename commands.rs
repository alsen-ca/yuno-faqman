pub enum Command {
    NewTag,
    NewThema,
    GetThema(String),
    NewQa,
    Exit,
    Help,
    Clear,
    Unknown,
}

pub fn parse(input: &str) -> Command {
    let input = input.trim();
    if input.to_lowercase().starts_with("get thema") {
        let toSearch = input["get thema ".len()..].trim();
        let toSearch = toSearch.trim_matches('"');
        Command::GetThema(toSearch.to_string())
    } else {
        match input.to_lowercase().as_str() {
            "new tag" => Command::NewTag,
            "new thema" => Command::NewThema,
            "new qa" => Command::NewQa,
            "exit" | "e" => Command::Exit,
            "help" | "h" => Command::Help,
            "clear" | "c"=> Command::Clear,
            _ => Command::Unknown,
        }
    }
}
