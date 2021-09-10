use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
use db::helper::GeneralError;
use db::sqlite;
use serde::{Deserialize, Serialize};

use r2d2_sqlite::{self, SqliteConnectionManager};

mod db;
mod kv;
use crate::db::{add_document, edit_document, remove_document, search_document};
use crate::db::sqlite::{init, Pool};
use crate::db::helper::{Document};
use crate::kv::RocksDB;

#[derive(Serialize, Deserialize)]
struct Response {
    response: String
}

#[derive(Serialize, Deserialize)]
struct SearchRequest {
    id: i64,
    query: String,
    peri_text_length: i64,
    stop_after_one: bool
}

#[post("/add")]
async fn add(rocks_db: web::Data<RocksDB>, sqlite_db: web::Data<Pool>, doc: web::Json<Document>)-> Result<HttpResponse, Error> {
    add_document(rocks_db, sqlite_db, Document { id: doc.id, name: doc.name.to_string(), text: doc.text.to_string() })?;

    Ok(HttpResponse::Ok().json(Response {
        response: "Successfully saved to the database.".to_string()
    }))
}

#[post("/edit")]
async fn edit(rocks_db: web::Data<RocksDB>, sqlite_db: web::Data<Pool>, doc: web::Json<Document>)-> Result<HttpResponse, Error> {
    edit_document(rocks_db, sqlite_db, Document { id: doc.id, name: doc.name.to_string(), text: doc.text.to_string() })?;

    Ok(HttpResponse::Ok().json(Response {
        response: "Successfully edit Document.".to_string()
    }))
}

#[post("/remove")]
async fn remove(rocks_db: web::Data<RocksDB>, sqlite_db: web::Data<Pool>, doc: web::Json<Document>)-> Result<HttpResponse, Error> {
    remove_document(rocks_db, sqlite_db, Document { id: doc.id, name: doc.name.to_string(), text: doc.text.to_string() })?;

    Ok(HttpResponse::Ok().json(Response {
        response: "Successfully removed from the database".to_string()
    }))
}

#[post("/search")]
async fn search(rocks_db: web::Data<RocksDB>, sqlite_db: web::Data<Pool>, req: web::Json<SearchRequest>)-> Result<HttpResponse, Error> {
    let results = search_document(rocks_db, sqlite_db,  req.query.to_string())?;

    Ok(HttpResponse::Ok().json(results))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rocks_db: kv::RocksDB = kv::KVStore::init("data/rocks");

    let manager = SqliteConnectionManager::file("./data/sqlite.db");
    let sqlite_db = Pool::new(manager).unwrap();
    init(sqlite_db.get().unwrap()).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(rocks_db.clone())
            .data(sqlite_db.clone())
            .data(web::JsonConfig::default().limit(1024 * 1024 * 50))
            .service(add)
            .service(edit)
            .service(remove)
            .service(search)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}