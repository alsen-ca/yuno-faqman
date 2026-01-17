use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
pub struct Qa {
    pub question: String,
    pub question_weights: Vec<(String, f32)>,
    pub answer: String,
    pub lang: String
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
    lang: String,
}

pub async fn handle_new_qa(qa: Qa) {
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

    let json = match serde_json::to_string_pretty(&api_qa) {
        Ok(j) => j,
        Err(e) => {
            println!("Failed to serialize qa: {}", e);
            return;
        }
    };

    println!("{}", json);


    let client = reqwest::Client::new();
    let response = match client
        .post("http://127.0.0.1:8221/qa")
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
        println!("Qa sent successfully!");
    } else {
        println!("Failed to send qa: {:?}", response.status());
    }
}
/*
pub async fn handle_get_qa(question: String) {
    println!("\r\n--- Requestion QA Based on Question ---");
    let json = match serde_json::to_string_pretty(&question) {
        Ok(j) => j,
        Err(e) => {
            println!("Failed to serialize qa: {}", e);
            return;
        }
    };

    println!("{}", json);

    let client = reqwest::Client::new();
    let response = match client
        .get("http://127.0.0.1:8221/qa")
        .await
    {
        Ok(res) => res,
        Err(e) => {
            println!("Failed to recieve request: {}", e);
            return;
        }
    };

    if response.status().is_success() {
        println!("Qa recieved successfully!");
    } else {
        println!("Failed to recieve qa: {:?}", response.status());
    }
}*/
