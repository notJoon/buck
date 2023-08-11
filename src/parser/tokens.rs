//! tokens.rs
//! 
//! This module contains the tokens used by the parser and constructs the
//! BuckTokens enum, which is used by the parser to determine what command
//! the user is trying to execute.

pub enum BuckTokens {
    Get,
    Insert,
    Remove,
    Update,
    Type,
    Commit,
    Rollback,
    Exit,
    Clear,
    Shard,
    LPush,
    LPop,
    SAdd,
    SRem,
    SInter,
    HSet,
    Length,
    Unknown,
}

impl BuckTokens {
    pub fn from_str(token: &str) -> Self {
        let token = token.to_lowercase();

        match token.as_str() {
            "get" => BuckTokens::Get,
            "insert" => BuckTokens::Insert,
            "remove" => BuckTokens::Remove,
            "update" => BuckTokens::Update,
            "type" => BuckTokens::Type,
            "commit" => BuckTokens::Commit,
            "rollback" => BuckTokens::Rollback,
            "exit" => BuckTokens::Exit,
            "clear" => BuckTokens::Clear,
            "shard" => BuckTokens::Shard,
            "lpush" => BuckTokens::LPush,
            "lpop" => BuckTokens::LPop,
            "sadd" => BuckTokens::SAdd,
            "srem" => BuckTokens::SRem,
            "sinter" => BuckTokens::SInter,
            "hset" => BuckTokens::HSet,
            "len" => BuckTokens::Length,
            _ => BuckTokens::Unknown,
        }
    }
}