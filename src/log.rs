use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum BuckLog {
    InsertOk(String),
    GetOk(String),
    RemoveOk(String),
    UpdateOk(String),
    TransactionOk,
    RollbackOk,
    BackupOk,
}

impl fmt::Display for BuckLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuckLog::InsertOk(key) => write!(f, "[log] {key}"),
            BuckLog::GetOk(key) => write!(f, "[log] {key}"),
            BuckLog::RemoveOk(key) => write!(f, "[log] {key}"),
            BuckLog::UpdateOk(key) => write!(f, "[log] {key}"),
            BuckLog::TransactionOk => write!(f, "[log] Transaction committed"),
            BuckLog::RollbackOk => write!(f, "[log] Transaction rolled back"),
            BuckLog::BackupOk => write!(f, "[Success] Database backed up"),
        }
    }
}
