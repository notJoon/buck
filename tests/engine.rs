#[cfg(test)]
mod db_test {
    use buck::{engine::BuckDB, types::BuckTypes, errors::BuckEngineError, log::BuckLog};

    #[test]
    fn test_insert_remove() {
        let mut db = BuckDB::new();

        let key = "test";
        let value = BuckTypes::String("test".to_string());

        db.insert(key.to_string(), value);

        let new_value = BuckTypes::String("new_value".to_string());
        let result = db.update(key, new_value).unwrap();

        assert_eq!(result, BuckLog::UpdateOk(key.to_string()));

        db.remove(key).unwrap();

        let result = db.get(key);

        assert_eq!(result, Err(BuckEngineError::KeyNotFound("test".to_string())));
    }

    #[test]
    fn test_insert_update_remove() {
        let mut db = BuckDB::new();

        let (key, value) = ("test", BuckTypes::String("test".to_string()));

        db.insert(key.to_string(), value);

        let new_value = BuckTypes::String("new_value".to_string());
        let result = db.update(key, new_value).unwrap();

        assert_eq!(result, BuckLog::UpdateOk(key.to_string()));
    }

    #[test]
    fn test_get() {
        let mut db = BuckDB::new();

        let (key, value) = ("test", BuckTypes::String("test".to_string()));

        db.insert(key.to_string(), value);

        let result = db.get(key).unwrap();

        assert_eq!(result, &BuckTypes::String("test".to_string()));
    }
}