use serde::Serialize;
use serde_json;
use std::str::FromStr;

#[derive(Debug, Serialize)]
pub enum Language {
    En,
    De,
    Es
}

impl FromStr for Language {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en" => Ok(Language::En),
            "de" => Ok(Language::De),
            "es" => Ok(Language::Es),
            _ => Err(())
        }
    }
} 

#[derive(Debug, Serialize)]
pub struct Qa {
    pub question: String,
    pub answer: String,
    pub lang: Language
}

pub fn handle_new_qa(qa: Qa) {
    println!("\r\n--- QA RECEIVED BY CONTROLLER ---");

    let json = serde_json::to_string_pretty(&qa)
        .expect("failed to serialize QA");

    println!("{}", json);
}
