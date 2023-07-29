use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum BuckLog {
    InsertOk(String),
    GetOk(String),
    RemoveOk(String),
    UpdateOk(String),
    ClearTransactionOk,
    TransactionOk,
    RollbackOk,
    BackupOk,
    TypeOk(String, String),
    ShardingEnableOk,
}

impl fmt::Display for BuckLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuckLog::InsertOk(key) => write!(f, "[log] {key}"),
            BuckLog::GetOk(key) => write!(f, "[log] {key}"),
            BuckLog::RemoveOk(key) => write!(f, "[log] {key}"),
            BuckLog::UpdateOk(key) => write!(f, "[log] {key}"),
            BuckLog::ClearTransactionOk => write!(f, "[log] Transaction cleared"),
            BuckLog::TransactionOk => write!(f, "[log] Transaction committed"),
            BuckLog::RollbackOk => write!(f, "[log] Transaction rolled back"),
            BuckLog::BackupOk => write!(f, "[Success] Database backed up"),
            BuckLog::TypeOk(key, typ) => write!(f, "[Success] {key}: {typ}"),
            BuckLog::ShardingEnableOk => write!(f, "[Success] Sharding enabled"),
        }
    }
}
