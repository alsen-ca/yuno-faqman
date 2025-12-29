use serde::Serialize;
use serde_json;
use std::str::FromStr;

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
    pub question_weights: Vec<(String, f32)>,
    pub answer: String,
    pub lang: Language
}

#[derive(Debug, Serialize)]
struct ApiWeight {
    word: String,
    weight: f32,
}

#[derive(Serialize)]
pub struct ApiQa {
    question: String,
    question_weights: Vec<ApiWeight>,
    answer: String,
    lang: Language,
}

pub fn handle_new_qa(qa: Qa) {
    println!("\r\n--- QA RECEIVED BY CONTROLLER ---");

    let api_weights: Vec<ApiWeight> = qa.question_weights
        .into_iter()
        .map(|(word, weight)| ApiWeight { word, weight })
        .collect();

    let api_qa = ApiQa {
        question: qa.question,
        question_weights: api_weights,
        answer: qa.answer,
        lang: qa.lang,
    };

    let json = serde_json::to_string_pretty(&api_qa)
        .expect("failed to serialize QA");

    println!("{}", json);
}
