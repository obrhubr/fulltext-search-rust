use actix_web::{web};
use crate::kv::RocksDB;

use crate::db::helper::{GeneralError, SearchResults, SearchResult, Values, Value};

use self::helper::Document;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub mod helper;
pub mod rocks;
pub mod sqlite;

pub fn add_document(rocks: web::Data<RocksDB>, sqlite: web::Data<Pool>, doc: Document) -> Result<(), GeneralError> {
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

fn convert_to_search_results(sqlite: Connection, values: Vec<Values>, peri_text: i64) -> Result<SearchResults, GeneralError> {
    let mut results: SearchResults = SearchResults { results: Vec::new() };

    for v in values {
        for val in v.values {
            // Map value to peri_text
            let doc = sqlite::get_document_sqlite(&sqlite, val.id)?;

            let split_text = doc[0].text.split(' ').collect::<Vec<&str>>();
            let pt = &split_text[std::cmp::max(val.position-peri_text, 0) as usize..std::cmp::min(val.position+peri_text, split_text.len() as i64) as usize];

            results.results.push(SearchResult { id: val.id, peri_text: pt.join(&' '.to_string()), word: val.position.to_string() });
        }
    }

    Ok(results)
}

fn rank_search_results(values: Vec<Values>, peri_text: i64) -> Result<Vec<Values>, GeneralError> {
    // Concatenate results within range of each other
    

    // rank by number of results in range of each other

    Ok(values)
}

pub fn search_document(rocks: web::Data<RocksDB>, sqlite: web::Data<Pool>, query: String, peri_text: i64) -> Result<SearchResults, GeneralError> {
    let rocks_result = rocks::search_document_rocks(&rocks, query)?;

    let ranked = rank_search_results(rocks_result, peri_text)?;
    let res = convert_to_search_results(sqlite.get().unwrap(), ranked, peri_text)?;

    Ok(res)
}