use crate::engine::BuckDB;

#[derive(Debug)]
pub struct Transaction {
    pub db: BuckDB,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            db: BuckDB::new(),
        }
    }
}