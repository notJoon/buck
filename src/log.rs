//! log.rs
//! 
//! This module contains the logs that print out to the user when they execute
//! a command.
//! 
//! For example, if the user types `insert key value` into CLI,
//! BuckDB must print out `[log] key` to the user. Which is constructed by the
//! BuckLog::InsertOk(String) enum.

use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum BuckLog {
    InsertOk(String),
    GetOk(String),
    RemoveOk(String),
    UpdateOk(String),
    ListPopOk(String),
    HSetOk(usize),
    LengthOk(usize),
    ClearTransactionOk,
    TransactionOk,
    RollbackOk,
    BackupOk,
    TypeOk(String, String),
    ShardingEnableOk,
    SetsIntersectionOk(String, Vec<String>),
    ClearOk,
}

impl fmt::Display for BuckLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuckLog::InsertOk(key) => write!(f, "[log] {key}"),
            BuckLog::GetOk(key) => write!(f, "{key}"),
            BuckLog::RemoveOk(key) => write!(f, "[log] {key}"),
            BuckLog::UpdateOk(key) => write!(f, "[log] {key}"),
            BuckLog::ListPopOk(value) => write!(f, "(pop) {value}"),
            BuckLog::HSetOk(length) => write!(f, "(integer) {length}"),
            BuckLog::LengthOk(length) => write!(f, "(integer) {length}"),
            BuckLog::ClearTransactionOk => write!(f, "[log] Transaction cleared"),
            BuckLog::TransactionOk => write!(f, "[log] Transaction committed"),
            BuckLog::RollbackOk => write!(f, "[log] Transaction rolled back"),
            BuckLog::BackupOk => write!(f, "[Success] Database backed up"),
            BuckLog::TypeOk(key, typ) => write!(f, "({typ}) {key}"),
            BuckLog::ShardingEnableOk => write!(f, "[Success] Sharding enabled"),
            BuckLog::SetsIntersectionOk(key, values) => {
                write!(f, "({key}) {values:?}", values = values)
            }
            BuckLog::ClearOk => write!(f, ""),
        }
    }
}
