#[cfg(test)]
mod db_test {
    use buck::{engine::BuckDB, types::BuckTypes};

    #[test]
    fn insert_remove() {
        let mut db = BuckDB::new();

        let key = "test";
        let value = BuckTypes::String("test".to_string());

        db.insert(key.to_string(), value);

        let new_value = BuckTypes::String("new_value".to_string());

        let result = db.update(key, new_value);

        assert_eq!(result, "Updated value: String(\"new_value\")");

        db.remove(key);

        assert_eq!(db.get(key), None);
    }

    #[test]
    fn insert_update_remove() {
        let mut db = BuckDB::new();

        let (key, value) = ("test", BuckTypes::String("test".to_string()));

        db.insert(key.to_string(), value);

        let new_value = BuckTypes::String("new_value".to_string());
        let result = db.update(key, new_value);

        assert_eq!(result, "Updated value: String(\"new_value\")");

        db.remove(key);
    }

    #[test]
    fn get() {
        let mut db = BuckDB::new();

        let (key, value) = ("test", BuckTypes::String("test".to_string()));

        db.insert(key.to_string(), value);

        let result = db.get(key);

        assert_eq!(result, Some(&BuckTypes::String("test".to_string())));
    }
}