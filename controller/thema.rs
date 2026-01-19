use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Thema {
    pub title: String,
}

static API_URL: &str = "http://127.0.0.1:8221/thema";

pub async fn handle_new_thema(thema: Thema) {
    println!("\r\n--- THEMA RECEIVED BY CONTROLLER ---");

    let json = match serde_json::to_string_pretty(&thema) {
        Ok(j) => j,
        Err(e) => {
            println!("Failed to serialize thema: {}", e);
            return;
        }
    };

    println!("{}", json);
    
    let client = reqwest::Client::new();
    let response = match client
        .post("http://127.0.0.1:8221/thema")
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
        println!("Thema sent successfully!");
    } else {
        println!("Failed to send thema: {:?}", response.status());
    }
}

pub async fn handle_get_thema(title: String) {
    println!("Requestion GET for Thema {}", title);

    let url = if title == "all" {
        API_URL.to_string()
    } else {
        format!("{}?title={}", API_URL, title)
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

        if let Err(e) = parse_get_response(&body, title == "all") {
            println!("Failed to parse response: {}", e);
        }
    } else {
        println!("Failed to recieve thema: {:?}", response.status());
    }
}

fn parse_get_response(body: &str, is_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    if is_all {
        // List of Themas
        let themas: Vec<Thema> = serde_json::from_str(body)?;
        for (index, thema) in themas.iter().enumerate() {
            println!(
                "{}. {}",
                index + 1,
                thema.title
            );
        }
    } else {
        // Single Thema
        let _thema: Thema = serde_json::from_str(body)?;
        println!("{}", body);
    }
    Ok(())
}
