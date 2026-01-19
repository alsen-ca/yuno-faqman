use uuid::Uuid;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WholeThema {
    pub id: Uuid,
    pub title: String
}

lazy_static! {
    pub static ref THEMEN: Mutex<Vec<WholeThema>> = Mutex::new(Vec::new());
}

pub fn get_themas() -> Vec<WholeThema> {
    THEMEN.lock().unwrap().clone()
}
