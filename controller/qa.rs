use serde::{Serialize, Deserialize};
use serde_json;
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Qa {
    pub question: String,
    pub question_weights: Vec<(String, f32)>,
    pub answer: String,
    pub lang: String,
    pub thema_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QaRes {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Question")]
    pub question: String,
    #[serde(rename = "QuestionWeights")]
    pub question_weights: Vec<QuestionWeight>,
    #[serde(rename = "Answer")]
    pub answer: String,
    #[serde(rename = "Language")]
    pub lang: String,
    #[serde(rename = "ThemaID")]
    pub thema_id: String,
    #[serde(rename = "TagIDs")]
    pub tag_ids: Option<String>,
}

#[derive(Serialize)]
pub struct ApiQa {
    question: String,
    question_weights: Vec<QuestionWeight>,
    answer: String,
    lang: String,
    thema_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionWeight {
    pub word: String,
    pub weight: f32,
}

static API_URL: &str = "http://127.0.0.1:8221/qa";

pub async fn handle_new_qa(qa: Qa) {
    println!("\r\n--- QA RECEIVED BY CONTROLLER ---");
    println!("This is qa: {:?}", qa);

    let api_weights: Vec<QuestionWeight> = qa.question_weights
        .into_iter()
        .map(|(word, weight)| QuestionWeight { word, weight })
        .collect();

    let api_qa = ApiQa {
        question: qa.question,
        question_weights: api_weights,
        answer: qa.answer,
        lang: qa.lang,
        thema_id: qa.thema_id,
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
        println!("Qa sent successfully!");
    } else {
        println!("Failed to send qa: {:?}", response.status());
    }
}

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

    let url = API_URL;
    let client = reqwest::Client::new();
    let response = match client.get(url).send().await {
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

        if let Err(e) = parse_get_response(&body, question == "all") {
            println!("Failed to parse response: {}", e);
        }
    } else {
        println!("Failed to recieve qa: {:?}", response.status());
    }
}

fn parse_get_response(body: &str, is_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    if is_all {
        let qas: Vec<QaRes> = serde_json::from_str(body)?;
        for (index, qa) in qas.iter().enumerate() {
            println!(
                "{}. Question: {} \nAnswer: {}",
                index + 1,
                qa.question,
                qa.answer
            );
        }
    } else {
        println!("Printing single qa: {}", body);
        let _qa: Qa = serde_json::from_str(body)?;
        println!("{}", body);
    }
    Ok(())
}