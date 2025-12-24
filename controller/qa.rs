use serde::Serialize;
use serde_json;
use std::str::FromStr;
use std::collections::HashMap

#[derive(Debug, Serialize)]
pub enum Language {
    English,
    German,
    Spanish
}

impl FromStr for Language {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en" => Ok(Language::English),
            "de" => Ok(Language::German),
            "es" => Ok(Language::Spanish),
            _ => Err(())
        }
    }
} 

#[derive(Debug, Serialize)]
pub struct Qa {
    pub question: String,
    pub question_weights: HashMap<String, f32>,
    pub answer: String,
    pub lang: Language
}

pub fn handle_new_qa(qa: Qa) {
    println!("\r\n--- QA RECEIVED BY CONTROLLER ---");

    let json = serde_json::to_string_pretty(&qa)
        .expect("failed to serialize QA");

    println!("{}", json);
}
