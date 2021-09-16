use actix_web::{web};
use crate::kv::RocksDB;

use crate::db::helper::{GeneralError, SearchResults, SearchResult, Values, Value, RankValue};
use std::cmp::Ordering;

use self::helper::Document;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub mod helper;
pub mod rocks;
pub mod sqlite;

pub fn add_document(rocks: web::Data<RocksDB>, sqlite: web::Data<Pool>, mut doc: Document) -> Result<(), GeneralError> {
    rocks::add_document_rocks(&rocks, &doc)?;
    sqlite::add_document_sqlite(sqlite.get().unwrap(), &doc)?;

    Ok(())
}

pub fn edit_document(rocks: web::Data<RocksDB>, sqlite: web::Data<Pool>, doc: Document) -> Result<(), GeneralError> {
    let old_doc = sqlite::get_document_sqlite(&sqlite.get().unwrap(), doc.id)?;

    rocks::update_document_rocks(&rocks, &old_doc[0], &doc)?;

    sqlite::update_document_sqlite(sqlite.get().unwrap(), doc)?;

    Ok(())
}

pub fn remove_document(rocks: web::Data<RocksDB>, sqlite: web::Data<Pool>, doc: Document) -> Result<(), GeneralError> {
    rocks::remove_document_rocks(&rocks, &doc)?;
    sqlite::remove_document_sqlite(sqlite.get().unwrap(), doc.id)?;

    Ok(())
}

fn convert_to_search_results(sqlite: Connection, values: Vec<RankValue>, peri_text: i64) -> Result<SearchResults, GeneralError> {
    let mut results: SearchResults = SearchResults { results: Vec::new() };

    for val in values {
        // Map value to peri_text
        let doc = sqlite::get_document_sqlite(&sqlite, val.id)?;

        let split_text = doc[0].text.split(' ').collect::<Vec<&str>>();
        let pt = &split_text[std::cmp::max(val.position-peri_text, 0) as usize..std::cmp::min(val.position+peri_text, split_text.len() as i64) as usize];

        results.results.push(SearchResult { id: val.id, peri_text: pt.join(&' '.to_string()), word: val.position.to_string() });
    }

    Ok(results)
}

fn rank_search_results(mut values: Vec<Value>, peri_text: i64) -> Result<Vec<RankValue>, GeneralError> {
    let mut results: Vec<RankValue> = Vec::new();  

    // Sort by book id and position
    values.sort_by(|a, b| {
        let ord = a.id.partial_cmp(&b.id).unwrap();
        if ord == Ordering::Equal {
            a.position.partial_cmp(&b.position).unwrap()
        } else {
            ord
        }
    });

    // Concatenate results within range of each other
    let mut prev_val = Value { id: -1, position: -1 };
    let mut w_num = 1;
    let mut dist = 0;

    for val in values {
        if val.id == prev_val.id && val.position <= prev_val.position + (peri_text - dist) {
            dist += val.position - prev_val.position;
            w_num += 1;
        } else {
            // If the id is -1 the loop is in the first iteration, and you don't want to add the placeholder value
            if prev_val.id == -1 { 
                prev_val = val; 
                continue;
            };

            let mid_val = RankValue { id: prev_val.id, position: (prev_val.position + (dist / 2)), word_num: w_num };

            // Reset values
            dist = 0;
            w_num = 1;

            results.push(mid_val);
        }

        prev_val = val;
    }

    // Add last val to ensure no result is left
    let final_val = RankValue { id: prev_val.id, position: (prev_val.position + (dist / 2)), word_num: w_num };
    results.push(final_val);

    // Sort by number of results in range of each other
    results.sort_by(|a, b| b.word_num.partial_cmp(&a.word_num).unwrap());

    Ok(results)
}

pub fn flatten_results(values: Vec<Values>) -> Vec<Value> {
    let mut flattened: Vec<Value> = Vec::new();

    // Flatten vec
    for vs in values {
        for v in vs.values {
            flattened.push(v);
        }
    }

    flattened
}

pub fn search_document(rocks: web::Data<RocksDB>, sqlite: web::Data<Pool>, query: String, peri_text: i64) -> Result<SearchResults, GeneralError> {
    let rocks_result = rocks::search_document_rocks(&rocks, query)?;

    if rocks_result.is_empty() {
        return Ok(SearchResults { results: Vec::new() })
    }

    let ranked = rank_search_results(flatten_results(rocks_result), peri_text)?;
    let res = convert_to_search_results(sqlite.get().unwrap(), ranked, peri_text)?;

    Ok(res)
}

pub fn search_one_document(rocks: web::Data<RocksDB>, sqlite: web::Data<Pool>, query: String, id: i64, peri_text: i64) -> Result<SearchResults, GeneralError> {
    let rocks_result = rocks::search_document_rocks(&rocks, query)?;

    if rocks_result.is_empty() {
        return Ok(SearchResults { results: Vec::new() })
    }

    let flattened = flatten_results(rocks_result);
    let filtered = flattened.into_iter().filter(|val| val.id == id).collect();
    let ranked = rank_search_results(filtered, peri_text)?;
    let res = convert_to_search_results(sqlite.get().unwrap(), ranked, peri_text)?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::db::{flatten_results, rank_search_results};
    use crate::db::helper::{RankValue, Value, Values};

    #[test]
    fn flatten_results_unit_test() {
        let mut values = Vec::new();

        let mut c = 0;
        for _i in 0..2 {
            let mut vals = Values { values: Vec::new() };
            for _j in 0..2 {
                vals.values.push(Value {id: c, position: c});
                c += 1;
            }
            values.push(vals);
        }

        let mut flattened = Vec::new();
        for i in 0..4 {
            flattened.push(Value { id: i, position: i });
        }

        assert_eq!(
            flatten_results(values), 
            flattened
        )
    }

    #[test]
    fn search_ranking_unit_test() {
        let values = vec![
            Value { id: 0 , position: 0},
            Value { id: 0 , position: 1},
            Value { id: 0 , position: 2},
            Value { id: 1 , position: 0},
            Value { id: 1 , position: 10},
            Value { id: 1 , position: 100},
            Value { id: 2 , position: 0},
            Value { id: 2 , position: 100},
        ];

        let ranked = vec![
            RankValue { id: 0 , position: 3, word_num: 3},
            RankValue { id: 1 , position: 15, word_num: 2},
            RankValue { id: 1 , position: 100, word_num: 1},
            RankValue { id: 2 , position: 0, word_num: 1},
            RankValue { id: 2 , position: 100, word_num: 1},
        ];

        assert_eq!(
            rank_search_results(values, 15).unwrap(),
            ranked
        )
    }
}