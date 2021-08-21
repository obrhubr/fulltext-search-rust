use actix_web::{Error, web};
use rusqlite::Connection;
use crate::kv::RocksDB;

use self::helper::Document;

pub mod helper;
pub mod rocks;
pub mod sqlite;

pub fn add_document(rocks: web::Data<RocksDB>, sqlite: Connection, doc: Document) -> Result<(), Error> {
    match rocks::add_document_rocks(&rocks, doc) {
        Ok(()) => {}
        Err(e) => {
            
        }
    };

    match sqlite::add_document_sqlite(sqlite, doc) {
        Ok(()) => {}
        Err(e) => {
            
        }
    }

    Ok(())
}