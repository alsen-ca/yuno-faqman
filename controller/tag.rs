use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub en_og: String,
    pub de_trans: String,
    pub es_trans: String,
}

static API_URL: &str = "http://127.0.0.1:8221/tag";

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
        .post(API_URL)
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

pub async fn handle_get_tag(lang: String, question: String) {
    println!("Requestion GET for Tag of Language: {}", lang);
    let url = match lang.as_str() {
        "en" => format!("{}?en_og={}", API_URL, question),
        "de" => format!("{}?de_trans={}", API_URL, question),
        "es" => format!("{}?es_trans={}", API_URL, question),
        "all" => format!("{}", API_URL),
        _ => {
            println!("Unsupported language: {}", lang);
            return;
        }
    };
    let client = reqwest::Client::new();
    let response = match client.get(&url).send().await {
        Ok(res) => res,
        Err(e) => {
            println!("Failed to recieve request: {}", e);
            return;
        }
    };
    if response.status().is_success() {
        let body = match response.text().await {
            Ok(b) => b,
            Err(e) => {
                println!("Failed to read response body: {}", e);
                return;
            }
        };

        if let Err(e) = parse_get_response(&body, lang == "all") {
            println!("Failed to parse response: {}", e);
        }
    } else {
        println!("Failed to recieve tag: {:?}", response.status());
    }
}


fn parse_get_response(body: &str, is_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    if is_all {
        let tags: Vec<Tag> = serde_json::from_str(body)?;
        println!("\nEnglish, German, Spanish");
        for (index, tag) in tags.iter().enumerate() {
            println!(
                "{}. {}, {}, {}",
                index + 1,
                tag.en_og,
                tag.de_trans,
                tag.es_trans
            );
        }
    } else {
        let _tag: Tag = serde_json::from_str(body)?;
        println!("{}", body);
    }
    Ok(())
}
