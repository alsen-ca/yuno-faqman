use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct Tag {
    pub en_og: String,
    pub de_trans: String,
    pub es_trans: String,
}

pub fn handle_new_tag(tag: Tag) {
    println!("\r\n--- TAG RECEIVED BY CONTROLLER ---");

    let json = serde_json::to_string_pretty(&tag)
        .expect("failed to serialize tag");

    println!("{}", json);
}
