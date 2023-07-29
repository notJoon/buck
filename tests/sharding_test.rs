#[cfg(test)]
mod sharding_tests {
    use buck::engine::BuckDB;
    use buck::types::types::BuckTypes;

    #[test]
    fn test_sharding_insert() {
        let mut db = BuckDB::new();
        db.enable_sharding(4).unwrap();

        db.insert("k1".to_owned(), BuckTypes::Integer(1)).unwrap();
        db.insert("k2".to_owned(), BuckTypes::String("string".to_owned()))
            .unwrap();
        db.insert("k3".to_owned(), BuckTypes::Boolean(true))
            .unwrap();
        db.insert("k4".to_owned(), BuckTypes::Integer(2)).unwrap();
        db.insert("k5".to_owned(), BuckTypes::String("string2".to_owned()))
            .unwrap();
        db.insert("k6".to_owned(), BuckTypes::Boolean(false))
            .unwrap();
        db.insert("k7".to_owned(), BuckTypes::Integer(3)).unwrap();
        db.insert("k8".to_owned(), BuckTypes::String("string3".to_owned()))
            .unwrap();
        db.insert("k9".to_owned(), BuckTypes::Boolean(true))
            .unwrap();
        db.insert("k10".to_owned(), BuckTypes::Integer(4))
            .unwrap();

        for i in 0..4 {
            println!("Shard {}: {:?}", i, db.get_shard_data(i));
        }
    }

    #[test]
    fn test_sharding_insert_commit_get_value() {
        let mut db = BuckDB::new();
        db.enable_sharding(4).unwrap();

        db.insert("k1".to_owned(), BuckTypes::Integer(1)).unwrap();
        db.insert("k2".to_owned(), BuckTypes::Integer(2)).unwrap(); 
        db.insert("k3".to_owned(), BuckTypes::Integer(3)).unwrap();
        db.insert("k4".to_owned(), BuckTypes::Integer(4)).unwrap();
        db.insert("k5".to_owned(), BuckTypes::Integer(5)).unwrap();

        db.commit().unwrap();

        assert_eq!(db.get("k1"), Ok(&BuckTypes::Integer(1)));
        assert_eq!(db.get("k2"), Ok(&BuckTypes::Integer(2)));
        assert_eq!(db.get("k3"), Ok(&BuckTypes::Integer(3)));
        assert_eq!(db.get("k4"), Ok(&BuckTypes::Integer(4)));
        assert_eq!(db.get("k5"), Ok(&BuckTypes::Integer(5)));
    }

    #[test]
    fn test_sharding_insert_update_commit_get() {
        let mut db = BuckDB::new();
        db.enable_sharding(4).unwrap();

        db.insert("k1".to_owned(), BuckTypes::Integer(1)).unwrap();
        db.insert("k2".to_owned(), BuckTypes::Integer(2)).unwrap();
        db.insert("k3".to_owned(), BuckTypes::Integer(3)).unwrap();
        db.insert("k4".to_owned(), BuckTypes::Integer(4)).unwrap();

        db.commit().unwrap();

        db.update("k1", BuckTypes::Integer(10)).unwrap();
        db.update("k2", BuckTypes::Integer(20)).unwrap();
        db.update("k3", BuckTypes::Integer(30)).unwrap();
        db.update("k4", BuckTypes::Integer(40)).unwrap();

        for i in 0..4 {
            println!("Shard {}: {:?}", i, db.get_shard_data(i));
        }

        assert_eq!(db.get("k1"), Ok(&BuckTypes::Integer(10)));
        assert_eq!(db.get("k2"), Ok(&BuckTypes::Integer(20)));
        assert_eq!(db.get("k3"), Ok(&BuckTypes::Integer(30)));
        assert_eq!(db.get("k4"), Ok(&BuckTypes::Integer(40)));
    }
}
