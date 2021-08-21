use actix_web::{post, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod db;

mod kv;
use crate::kv::RocksDB;

#[derive(Serialize, Deserialize)]
struct Response {
    response: String
}

#[derive(Serialize, Deserialize)]
struct SearchResponse {
    results: Vec<Values>
}

#[derive(Serialize, Deserialize)]
struct SearchRequest {
    id: i64,
    query: String,
    peri_text_length: i64,
    stop_after_one: bool
}

#[post("/add")]
async fn add(db: web::Data<RocksDB>, doc: web::Json<Document>)-> Result<HttpResponse, Error> {
    //add_document(&db, Document { id: doc.id, name: doc.name.to_string(), text: doc.text.to_string() });


    Ok(HttpResponse::Ok().json(Response {
        response: "Successfully saved to the database.".to_string()
    }))
}

/* #[post("/edit")]
async fn edit(db: web::Data<RocksDB>, doc: web::Json<Document>)-> Result<HttpResponse, std::io::Error> {
    update_document(&db, Document { id: doc.id, name: doc.name.to_string(), text: doc.text.to_string() });
    Ok(HttpResponse::Ok().json(Response {
        response: "Successfully saved to the database.".to_string()
    }))
} */

#[post("/remove")]
async fn remove(db: web::Data<RocksDB>, doc: web::Json<Document>)-> Result<HttpResponse, Error> {
    remove_document(&db, Document { id: doc.id, name: doc.name.to_string(), text: doc.text.to_string() });
    Ok(HttpResponse::Ok().json(Response {
        response: "Successfully saved to the database.".to_string()
    }))
}

#[post("/search")]
async fn search(db: web::Data<RocksDB>, req: web::Json<SearchRequest>)-> Result<HttpResponse, Error> {
    let results = search_document(&db,  req.query.to_string())?;
    Ok(HttpResponse::Ok().json(SearchResponse {
        results
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: kv::RocksDB = kv::KVStore::init("data/rocks");

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .data(web::JsonConfig::default().limit(1024 * 1024 * 50))
            .service(add)
            //.service(edit)
            .service(remove)
            .service(search)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}