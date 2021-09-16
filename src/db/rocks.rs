use crate::kv::{KVStore, RocksDB};
use actix_web::{web};
use bincode::{serialize, deserialize};

extern crate phf;
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

use super::helper::{Document, GeneralError, Value, Values, remove_punctuation};

pub fn search_document_rocks(db: &web::Data<RocksDB>, query: String) -> Result<Vec<Values>, GeneralError> {
    // Split the query text by words
    let split_query = query.split(' ');

    // Create vector to hold search results TODO: Replace by specific searchresults struct
    let mut search_results: Vec<Values> = Vec::new();

    for word in split_query {
        // Remove punctuation and whitespaces from word
        let normalised_word = remove_punctuation(word.to_string());

        if KEYWORDS.contains(&normalised_word) {
            continue;
        }

        // Check if word is indexed
        match db.find(normalised_word.as_bytes()) {
            Ok(Some(value)) => {
                // Deserialize values and push to results
                let decoded_values: Values = deserialize(&value[..]).unwrap();
                search_results.push(decoded_values);
            },
            Ok(None) => {
                // If word is not in db simply continue
                continue;
            },
            Err(e) => {
                return Err(GeneralError::Rocks(e));
            },
        }
    }

    Ok(search_results)
}

pub fn check_stopword(word: &str) -> bool {
    KEYWORDS.contains(word)
}

pub fn add_document_rocks(db: &web::Data<RocksDB>, doc: &Document) -> Result<(), GeneralError> {
    // Split the input text by words
    let split_text = doc.text.split(' ');

    let mut i: i64 = 0;
    for word in split_text {
        // Remove punctuation and whitespaces from word
        let normalised_word = remove_punctuation(word.to_string());

        // Check if stopword
        if check_stopword(&normalised_word) {
            i += 1;
            continue;
        }

        // Create array to contain the updates values array
        let mut new_values: Values = Values { values: Vec::new() };

        // Check if word already exists in db
        match db.find(normalised_word.as_bytes()) {
            Ok(Some(value)) => {
                // Deserialize the db result and append the new value to it if value already exists
                let mut decoded_values: Values = deserialize(&value[..]).unwrap();
                new_values.values.append(&mut decoded_values.values);
                // Push new value to the array
                new_values.values.push(Value { id: doc.id, position: i})
            },
            Ok(None) => {
                // Simply push new value to the array
                new_values.values.push(Value { id: doc.id, position: i});
            },
            Err(e) => {
                return Err(GeneralError::Rocks(e));
            },
        }

        // Serialize values that contain the new value and put to db
        db.save(normalised_word.as_bytes(), &serialize(&new_values).unwrap()).unwrap();

        i += 1;
    }

    Ok(())
}

pub fn remove_document_rocks(db: &web::Data<RocksDB>, doc: &Document) -> Result<(), GeneralError> {
    // Loop through all words in document and delete from db
    let split_text = doc.text.split(' ');

    for word in split_text {
        // Remove punctuation and whitespaces from word
        let normalised_word = remove_punctuation(word.to_string());

        // Create array to contain the updated values array
        let mut new_values: Values = Values { values: Vec::new() };

        // Check if word already exists in db
        match db.find(normalised_word.as_bytes()) {
            Ok(Some(value)) => {
                // Deserialize the db result and remove the reference to document
                let mut decoded_values: Values = deserialize(&value[..]).unwrap();
                new_values.values.append(&mut decoded_values.values);
                // Remove reference to document
                new_values.values = new_values.values
                .into_iter()
                .filter(|val| val.id != doc.id)
                .collect();
            },
            Ok(None) => {},
            Err(e) => {
                return Err(GeneralError::Rocks(e))
            },
        }

        // Serialize values that contain the new value and put to db
        db.save(normalised_word.as_bytes(), &serialize(&new_values).unwrap())?;
    }

    Ok(())
}

pub fn update_document_rocks(db: &web::Data<RocksDB>, old_doc: &Document, new_doc: &Document) -> Result<(), GeneralError> {
    remove_document_rocks(&db, &old_doc)?;
    add_document_rocks(&db, &new_doc)?;

    Ok(())
}