use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub created_at: DateTime<Local>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRequest {
    pub name: String,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub id: String,
    pub created_at: DateTime<Local>,
    pub name: String,
    pub data: Data,
}

pub type Data = HashMap<String, Option<Entry>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Entry {
    Data(Data),
    String(String),
    Int(i64),
    Float(f64),
    Vec(Vec<Entry>),
    Bool(bool),
}
