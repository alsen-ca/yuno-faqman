use uuid::Uuid;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WholeTag {
    pub id: Uuid,
    pub en_og: String,
    pub de_trans: String,
    pub es_trans: String,
}

lazy_static! {
    pub static ref TAGS: Mutex<Vec<WholeTag>> = Mutex::new(Vec::new());
}

pub fn get_tags() -> Vec<WholeTag> {
    TAGS.lock().unwrap().clone()
}
