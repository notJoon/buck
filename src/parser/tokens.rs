pub enum BuckTokens {
    Get,
    Insert,
    Remove,
    Update,
    Type,
    Commit,
    Rollback,
    Exit,
    Shard,
    LPush,
    LPop,
    SAdd,
    SRem,
    SInter,
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
            "shard" => BuckTokens::Shard,
            "lpush" => BuckTokens::LPush,
            "lpop" => BuckTokens::LPop,
            "sadd" => BuckTokens::SAdd,
            "srem" => BuckTokens::SRem,
            "sinter" => BuckTokens::SInter,
            "len" => BuckTokens::Length,
            _ => BuckTokens::Unknown,
        }
    }
}