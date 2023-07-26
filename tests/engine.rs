#[cfg(test)]
mod db_test {
    use buck::{
        engine::{BuckDB, TransactionStatus},
        errors::BuckEngineError,
        log::BuckLog,
        types::BuckTypes,
    };

    #[test]
    fn test_insert_remove() {
        let mut db = BuckDB::new();

        let key = "test";
        let value = BuckTypes::String("test".to_string());

        db.insert(key.to_string(), value).unwrap();

        let new_value = BuckTypes::String("new_value".to_string());
        let result = db.update(key, new_value).unwrap();

        assert_eq!(result, BuckLog::UpdateOk(key.to_string()));

        db.remove(key).unwrap();

        let result = db.get(key);

        assert_eq!(
            result,
            Err(BuckEngineError::KeyNotFound("test".to_string()))
        );
    }

    #[test]
    fn test_insert_update_remove() {
        let mut db = BuckDB::new();

        let (key, value) = ("test", BuckTypes::String("test".to_string()));

        db.insert(key.to_string(), value).unwrap();

        let new_value = BuckTypes::String("new_value".to_string());
        let result = db.update(key, new_value).unwrap();

        assert_eq!(result, BuckLog::UpdateOk(key.to_string()));

        // commit transaction
        assert_eq!(db.commit(), Ok(BuckLog::TransactionOk));

        let (key2, value2) = ("test2", BuckTypes::String("test2".to_string()));

        db.insert(key2.to_string(), value2).unwrap();

        // check DB's status
        assert_eq!(db.status, buck::engine::TransactionStatus::Uncommitted);
        assert_eq!(db.data.len(), 1);

        db.commit().unwrap();

        // check DB's status
        assert_eq!(db.status, buck::engine::TransactionStatus::Committed);

        // check DB data length
        assert_eq!(db.data.len(), 2);
    }

    #[test]
    fn test_get() {
        let mut db = BuckDB::new();

        let (key, value) = ("test", BuckTypes::String("test".to_string()));

        db.insert(key.to_string(), value).unwrap();

        let result = db.get(key).unwrap();

        // take value from data before commit
        assert_eq!(result, &BuckTypes::String("test".to_string()));
        // commit transaction
        assert_eq!(db.commit(), Ok(BuckLog::TransactionOk));
        // after commit, take value from data
        assert_eq!(db.get(key), Ok(&BuckTypes::String("test".to_string())));
    }

    #[test]
    fn test_transaction_commit() {
        let mut db = BuckDB::new();
        db.begin_transaction();
        db.insert("key1".to_string(), BuckTypes::String("value1".to_string()))
            .unwrap();

        // take value from uncommitted_data
        assert_eq!(db.get("key1"), Ok(&BuckTypes::String("value1".to_string())));

        // commit transaction
        assert_eq!(db.commit(), Ok(BuckLog::TransactionOk));

        // after commit, take value from data
        assert_eq!(db.get("key1"), Ok(&BuckTypes::String("value1".to_string())));
    }

    #[test]
    fn test_multiple_data_committing_at_once() {
        let mut db = BuckDB::new();

        db.begin_transaction();

        db.insert("key1".to_string(), BuckTypes::String("value1".to_string()))
            .unwrap();
        db.insert("key2".to_string(), BuckTypes::String("value2".to_string()))
            .unwrap();
        db.insert("key3".to_string(), BuckTypes::String("value3".to_string()))
            .unwrap();
        db.insert("key4".to_string(), BuckTypes::String("value4".to_string()))
            .unwrap();

        assert_eq!(db.data.len(), 0);
        assert_eq!(db.commit(), Ok(BuckLog::TransactionOk));
        assert_eq!(db.data.len(), 4);
    }

    #[test]
    fn test_data_rollback_on_abort() {
        let mut db = BuckDB::new();

        db.begin_transaction();

        db.insert("key1".to_string(), BuckTypes::String("value1".to_string()))
            .unwrap();
        db.insert("key2".to_string(), BuckTypes::String("value2".to_string()))
            .unwrap();
        db.insert("key3".to_string(), BuckTypes::String("value3".to_string()))
            .unwrap();

        db.commit().unwrap();

        db.insert("key1".to_string(), BuckTypes::String("value4".to_string()))
            .unwrap();
        assert_eq!(db.get("key1"), Ok(&BuckTypes::String("value4".to_string())));

        db.status = TransactionStatus::Abort;

        assert_eq!(db.abort(), Ok(BuckLog::RollbackOk));
        assert_eq!(db.data.len(), 3);
        assert_eq!(db.get("key1"), Ok(&BuckTypes::String("value1".to_string())));
    }
}
