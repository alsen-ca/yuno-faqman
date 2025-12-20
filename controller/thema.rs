use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct Thema {
    pub title: String,
}

pub fn handle_new_thema(thema: Thema) {
    println!("\r\n--- THEMA RECEIVED BY CONTROLLER ---");

    let json = serde_json::to_string_pretty(&thema)
        .expect("failed to serialize thema");

    println!("{}", json);
}
