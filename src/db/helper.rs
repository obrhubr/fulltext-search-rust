use serde::{Deserialize, Serialize};

#[derive(serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Debug)]
pub struct Value {
    pub id: i64,
    pub position: i64
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Debug)]
pub struct Values {
    pub values: Vec<Value>
}

pub fn remove_punctuation(mut word: String) -> String {
    word.retain(|c| !c.is_whitespace() || !c.is_ascii_punctuation() || c.is_alphanumeric());
    word
}

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub id: i64,
    pub name: String,
    pub text: String
}