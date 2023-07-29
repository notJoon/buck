use crate::{engine::BuckDB, errors::BuckEngineError, log::BuckLog};
use crate::types::types::BuckTypes;

#[derive(Debug, PartialEq)]
pub enum BuckQuery {
    Get(Vec<String>),
    Insert(String, BuckTypes),
    Update(String, BuckTypes),
    Remove(Vec<String>),
    Shard(usize),
    Type(String),
    // list things
    Lpush(String, Vec<BuckTypes>),
    //TODO Commit and Rollback may be take db name as argument
    Commit,
    Rollback,
    Exit,
    Unknown,
}

impl BuckQuery {
    pub fn execute(self, query: &str, db: &mut BuckDB) -> Result<BuckLog, BuckEngineError> {
        match self {
            BuckQuery::Get(keys) => {
                let mut results = Vec::new();

                for key in keys {
                    let value = db.get(&key).unwrap();
                    results.push(format!("{}: {}", key, value));
                }

                Ok(BuckLog::GetOk(results.join("\n")))
            }
            BuckQuery::Insert(key, value) => {
                db.insert(key, value).unwrap();

                Ok(BuckLog::InsertOk(query.to_owned()))
            }
            BuckQuery::Remove(keys) => {
                for key in keys {
                    db.remove(&key).unwrap();
                }

                Ok(BuckLog::RemoveOk(query.to_owned()))
            }
            BuckQuery::Update(key, value) => {
                db.update(&key, value).unwrap();

                Ok(BuckLog::UpdateOk(query.to_owned()))
            }
            BuckQuery::Type(key) => {
                let typ = db.type_of(&key).unwrap();

                Ok(BuckLog::TypeOk(key, typ.to_string()))
            }
            BuckQuery::Commit => {
                db.commit().unwrap();

                Ok(BuckLog::TransactionOk)
            }
            BuckQuery::Rollback => {
                db.abort().unwrap();

                Ok(BuckLog::RollbackOk)
            }
            BuckQuery::Exit => {
                std::process::exit(0);
            }
            BuckQuery::Shard(num_shards) => {
                db.enable_sharding(num_shards).unwrap();

                Ok(BuckLog::ShardingEnableOk)
            }

            // list things
            BuckQuery::Lpush(key, values) => {
                for value in values {
                    db.l_push(key.clone(), value).unwrap();
                }

                Ok(BuckLog::InsertOk(query.to_owned()))
            }

            _ => {
                unimplemented!("Not implemented yet")
            }
        }
    }
}
