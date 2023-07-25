use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum BuckLog {
    InsertOk(String),
    GetOk(String),
    RemoveOk(String),
    UpdateOk(String),
    TransactionOk,
    RollbackOk,
}

impl fmt::Display for BuckLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuckLog::InsertOk(key) => write!(f, "[log] {key} inserted into database"),
            BuckLog::GetOk(key) => write!(f, "[log] {key} retrieved from database"),
            BuckLog::RemoveOk(key) => write!(f, "[log] {key} removed from database"),
            BuckLog::UpdateOk(key) => write!(f, "[log] {key} updated in database"),
            BuckLog::TransactionOk => write!(f, "[log] Transaction committed"),
            BuckLog::RollbackOk => write!(f, "[log] Transaction rolled back"),
        }
    }
}