use serde::{Deserialize, Serialize};
use derive_more::{Display, Error};

#[derive(serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Debug)]
pub struct Value {
    pub id: i64,
    pub position: i64
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, PartialEq, Debug)]
pub struct RankValue {
    pub id: i64,
    pub position: i64,
    pub word_num: i64
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

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub id: i64,
    pub peri_text: String,
    pub word: String
}

#[derive(Serialize, Deserialize)]
pub struct SearchResults {
    pub results: Vec<SearchResult>
}

#[derive(Debug, Display, Error)]
pub enum GeneralError {
    Rusqlite(rusqlite::Error),
    ActixWeb(actix_web::Error),
    Rocks(rocksdb::Error)
}

impl From<rusqlite::Error> for GeneralError {
    fn from(error: rusqlite::Error) -> Self {
        GeneralError::Rusqlite(error)
    }
}

impl From<actix_web::Error> for GeneralError {
    fn from(error: actix_web::Error) -> Self {
        GeneralError::ActixWeb(error)
    }
}

impl From<rocksdb::Error> for GeneralError {
    fn from(error: rocksdb::Error) -> Self {
        GeneralError::Rocks(error)
    }
}

impl actix_web::error::ResponseError for GeneralError {

}