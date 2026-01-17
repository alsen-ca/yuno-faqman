use serde::Serialize;
use serde_json;
use reqwest;

#[derive(Debug, Serialize)]
pub struct Tag {
    pub en_og: String,
    pub de_trans: String,
    pub es_trans: String,
}

pub async fn handle_new_tag(tag: Tag) {
    println!("\r\n--- TAG RECEIVED BY CONTROLLER ---");

    let json = match serde_json::to_string_pretty(&tag) {
        Ok(j) => j,
        Err(e) => {
            println!("Failed to serialize tag: {}", e);
            return;
        }
    };

    println!("{}", json);
    
    let client = reqwest::Client::new();
    let response = match client
        .post("http://127.0.0.1:8221/tag")
        .header("Content-Type", "application/json")
        .body(json)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            println!("Failed to send request: {}", e);
            return;
        }
    };

    if response.status().is_success() {
        println!("Tag sent successfully!");
    } else {
        println!("Failed to send tag: {:?}", response.status());
    }
}
