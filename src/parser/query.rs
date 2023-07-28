use crate::{types::BuckTypes, engine::BuckDB, errors::BuckEngineError, log::BuckLog};

#[derive(Debug, PartialEq)]
pub enum BuckQuery {
    Get(Vec<String>),
    Insert(String, BuckTypes),
    Update(String, BuckTypes),
    Remove(Vec<String>),
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
            _ => {
                unimplemented!("Not implemented yet")
            }
        }
    }
}