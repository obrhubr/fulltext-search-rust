use rusqlite::{Result, Error};

use super::helper::Document;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub fn init(db: Connection) -> Result<(), Error> {
    db.execute(
        "create table if not exists documents (
            id integer primary key,
            doc_id text not null,
            doc_name text not null,
            doc_text text not null
        )",
        [],
    )?;

    Ok(())
}

pub fn add_document_sqlite(db: Connection, doc: &Document) -> Result<(), Error> {
    db.execute(
        "INSERT INTO documents (doc_id, doc_name, doc_text) values (?1, ?2, ?3)",
        &[&doc.id.to_string(), &doc.name.to_string(), &doc.text.to_string()],
    )?;

    Ok(())
}

pub fn update_document_sqlite(db: Connection, doc: Document) -> Result<(), Error> {
    if doc.name == "" {
        db.execute(
            "UPDATE documents SET doc_text = ?1 WHERE doc_id = ?2",
            &[&doc.text.to_string(), &doc.id.to_string()],
        )?;
    } else if doc.text == "" {
        db.execute(
            "UPDATE documents SET doc_name = ?1 WHERE doc_id = ?2",
            &[&doc.name.to_string(), &doc.id.to_string()],
        )?;
    } else {
        db.execute(
            "UPDATE documents SET doc_name = ?1, doc_text = ?2 WHERE doc_id = ?3",
            &[&doc.name.to_string(), &doc.text.to_string(), &doc.id.to_string()],
        )?;
    }

    Ok(())
}

pub fn remove_document_sqlite(db: Connection, id: i64) -> Result<(), Error> {
    db.execute(
        "DELETE FROM documents WHERE doc_id = ?",
        &[&id],
    )?;

    Ok(())
}

pub fn get_document_sqlite(db: &Connection, id: i64) -> Result<Vec<Document>, Error> {
    let mut stmt = db.prepare(
        "SELECT * FROM documents WHERE doc_id = ?",
    )?;

    let results_iter = stmt.query_map([id.to_string()], |row| {
        Ok(Document {
            id: row.get::<usize, String>(1)?.parse::<i64>().unwrap(),
            name: row.get(2)?,
            text: row.get(3)?
        })
    })?;

    let mut results: Vec<Document> = Vec::new();
    for result in results_iter {
        results.push(result.unwrap());
    }

    Ok(results)
}