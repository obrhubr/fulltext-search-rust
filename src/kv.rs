use rocksdb::DB;
use rocksdb::Error;
use std::sync::Arc;

pub trait KVStore {
    fn init(file_path: &str) -> Self;
    fn save(&self, k: &[u8], v: &[u8]) -> Result<(), Error>;
    fn find(&self, k: &[u8]) -> Result<Option<Vec<u8>>, Error>;
}

#[derive(Clone)]
pub struct RocksDB {
    db: Arc<DB>,
}


impl KVStore for RocksDB {
    fn init(file_path: &str) -> Self {
        RocksDB { db: Arc::new(DB::open_default(file_path).unwrap()) }
    }

    fn save(&self, k: &[u8], v: &[u8]) -> Result<(), Error> {
        self.db.put(k, v)
    }

    fn find(&self, k: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        self.db.get(k)
    }
}