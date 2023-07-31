use crate::types::types::BuckTypes;
use crate::{engine::BuckDB, errors::BuckEngineError, log::BuckLog};

#[derive(Debug, PartialEq)]
pub enum BuckQuery {
    Get(Vec<String>),
    Insert(String, BuckTypes),
    Update(String, BuckTypes),
    Remove(Vec<String>),
    Shard(usize),
    Type(String),
    // list things
    LPush(String, Vec<BuckTypes>),
    LPop(String),
    // sets type things
    SAdd(String, Vec<BuckTypes>),
    SRem(String, Vec<BuckTypes>),
    SInter(String, Vec<String>),
    // for all collection types
    Len(String),
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
            BuckQuery::LPush(key, values) => {
                for value in values {
                    db.l_push(key.clone(), value).unwrap();
                }

                Ok(BuckLog::InsertOk(query.to_owned()))
            }
            BuckQuery::LPop(key) => {
                let value = db.l_pop(&key).unwrap();

                Ok(BuckLog::GetOk(format!("{}: {}", key, value)))
            }
            // sets type things
            BuckQuery::SAdd(key, values) => {
                for value in values {
                    db.s_add(key.clone(), value).unwrap();
                }

                Ok(BuckLog::InsertOk(query.to_owned()))
            }
            BuckQuery::SInter(target, others) => {
                let target = db.s_inter(target, others.clone()).unwrap();
                
                Ok(BuckLog::SetsIntersectionOk(target.to_string(), others))
            }
            BuckQuery::SRem(key, values) => {
                for value in values {
                    db.s_rem(key.clone(), value).unwrap();
                }

                Ok(BuckLog::RemoveOk(query.to_owned()))
            }
            BuckQuery::Len(key) => {
                let length = db.get_collections_length(key.clone()).unwrap();

                Ok(BuckLog::LengthOk(length))
            }
            _ => {
                unimplemented!("Not implemented yet")
            }
        }
    }
}
