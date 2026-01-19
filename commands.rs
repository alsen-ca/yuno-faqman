pub enum Command {
    NewTag,
    GetTag { lang: String, question: String},
    NewThema,
    GetThema(String),
    NewQa,
    GetQa(String),
    Exit,
    Help,
    Clear,
    Unknown,
}

pub fn parse(input: &str) -> Command {
    let input_og = input.trim();
    let input = input.trim().to_lowercase();

    fn extract_search_term(input: &str, prefix: &str) -> Option<String> {
        input.strip_prefix(prefix)
            .map(|s| s.trim().trim_matches('"').to_string())
            .filter(|s| !s.is_empty())
    }

    // Split input into words
    let words: Vec<&str> = input.split_whitespace().collect();

    match words.as_slice() {
        ["get", "tag", lang, ..] if ["en", "de", "es", "all"].contains(lang) => {
            // Extract question after language
            let question = input_og
                .strip_prefix(&format!("get tag {} ", lang))
                .map(|s| s.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_default();

            Command::GetTag {
                lang: lang.to_string(),
                question,
            }
        }
        ["get", "thema", ..] => extract_search_term(&input_og, "get thema ").map_or(Command::Unknown, Command::GetThema),
        ["get", "qa", ..] => extract_search_term(&input_og, "get qa ").map_or(Command::Unknown, Command::GetQa),

        ["new", "tag"] => Command::NewTag,
        ["new", "thema"] => Command::NewThema,
        ["new", "qa"] => Command::NewQa,
        ["exit"] | ["e"] => Command::Exit,
        ["help"] | ["h"] => Command::Help,
        ["clear"] | ["c"] => Command::Clear,
        _ => Command::Unknown,
    }
}
