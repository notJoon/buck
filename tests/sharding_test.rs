#[cfg(test)]
mod sharding_tests {
    use buck::engine::BuckDB;
    use buck::types::BuckTypes;

    #[test]
    fn test_sharding_insert() {
        let mut db = BuckDB::new();
        db.enable_sharding(4).unwrap();

        db.insert("key1".to_owned(), BuckTypes::Integer(1)).unwrap();
        db.insert("key2".to_owned(), BuckTypes::String("string".to_owned())).unwrap();
        db.insert("key3".to_owned(), BuckTypes::Boolean(true)).unwrap();
        db.insert("key4".to_owned(), BuckTypes::Integer(2)).unwrap();
        db.insert("key5".to_owned(), BuckTypes::String("string2".to_owned())).unwrap();
        db.insert("key6".to_owned(), BuckTypes::Boolean(false)).unwrap();
        db.insert("key7".to_owned(), BuckTypes::Integer(3)).unwrap();
        db.insert("key8".to_owned(), BuckTypes::String("string3".to_owned())).unwrap();
        db.insert("key9".to_owned(), BuckTypes::Boolean(true)).unwrap();
        db.insert("key10".to_owned(), BuckTypes::Integer(4)).unwrap();

        for i in 0..4 {
            println!("Shard {}: {:?}", i, db.get_shard_data(i));
        }
    }
}