#[cfg(test)]
mod query_tests {
    use std::collections::{HashMap, HashSet};

    use buck::parser::errors::BuckParserError;
    use buck::parser::parse::parse_query;
    use buck::parser::query::BuckQuery::{Get, Insert, Remove, Update};
    use buck::types::BuckTypes;

    #[test]
    fn test_parse_insert() {
        let query = "INSERT key 1";
        let result = parse_query(query);
        assert_eq!(result, Ok(Insert("key".to_string(), BuckTypes::Integer(1))));

        let query = "INSERT key 1_000_000_000_000";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert("key".to_string(), BuckTypes::Integer(1000000000000)))
        );

        let query = "INSERT key 1.0";
        let result = parse_query(query);
        assert_eq!(result, Ok(Insert("key".to_string(), BuckTypes::Float(1.0))));

        let query = "INSERT key 1_000_000_000_000.012_345_678_900";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert(
                "key".to_string(),
                BuckTypes::Float(1000000000000.0123456789)
            ))
        );

        let query = "INSERT key true";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert("key".to_string(), BuckTypes::Boolean(true)))
        );

        let query = "INSERT key True";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert(
                "key".to_string(),
                BuckTypes::Unknown("True".to_string())
            ))
        );

        let query = "INSERT key True True";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert(
                "key".to_string(),
                BuckTypes::Unknown("True True".to_string())
            ))
        );

        let query = "INSERT key false";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert("key".to_string(), BuckTypes::Boolean(false)))
        );

        let query = "INSERT key \"test\"";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert(
                "key".to_string(),
                BuckTypes::String("test".to_string())
            ))
        );

        let query = "INSERT key \"FOO BAR BAZ\"";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert(
                "key".to_string(),
                BuckTypes::String("FOO BAR BAZ".to_string())
            ))
        );

        let query = "INSERT key \"foo bar baz\"";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert(
                "key".to_string(),
                BuckTypes::String("foo bar baz".to_string())
            ))
        );

        let query = "INSERT key q1w2e3r4t5!!";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Insert(
                "key".to_string(),
                BuckTypes::Unknown("q1w2e3r4t5!!".to_string())
            ))
        );

        let invalid_query = "INSERT key";
        let result = parse_query(invalid_query);
        assert_eq!(
            result,
            Err(BuckParserError::InvalidQueryCommand(
                invalid_query.to_string()
            ))
        );

        let invalid_number_key = "INSERT 1 2";
        let result = parse_query(invalid_number_key);
        assert_eq!(
            result,
            Err(BuckParserError::InvalidKey("1".to_string()))
        );
    }

    #[test]
    fn test_parse_get() {
        let query = "GET key";
        let result = parse_query(query);
        assert_eq!(result, Ok(Get(vec!["key".to_string()])));

        let multiple_keys = "GET key1 key2 key3 key4 key5 key6 key7";
        let result = parse_query(multiple_keys);
        assert_eq!(
            result,
            Ok(Get(vec![
                "key1".to_string(),
                "key2".to_string(),
                "key3".to_string(),
                "key4".to_string(),
                "key5".to_string(),
                "key6".to_string(),
                "key7".to_string(),
            ]))
        );

        let query = "GET";
        let result = parse_query(query);
        assert_eq!(
            result,
            Err(BuckParserError::InvalidQueryCommand(query.to_string()))
        );

        let invalid_number_key = "GET 1 2";
        let result = parse_query(invalid_number_key);
        assert_eq!(
            result,
            Err(BuckParserError::InvalidKey("1, 2".to_string()))
        );
    }

    #[test]
    fn test_parse_update() {
        let query = "UPDATE key 1";
        let result = parse_query(query);
        assert_eq!(result, Ok(Update("key".to_string(), BuckTypes::Integer(1))));

        let query = "UPDATE key 1_000_000_000_000";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update("key".to_string(), BuckTypes::Integer(1000000000000)))
        );

        let query = "UPDATE key 1.0";
        let result = parse_query(query);
        assert_eq!(result, Ok(Update("key".to_string(), BuckTypes::Float(1.0))));

        let query = "UPDATE key 1_000_000_000_000.012_345_678_900";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_string(),
                BuckTypes::Float(1000000000000.0123456789)
            ))
        );

        let query = "UPDATE key true";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update("key".to_string(), BuckTypes::Boolean(true)))
        );

        let query = "UPDATE key True";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_string(),
                BuckTypes::Unknown("True".to_string())
            ))
        );

        let query = "UPDATE key True True";
        let result = parse_query(query);
        assert_eq!(
            result,
            Err(BuckParserError::UpdateValueContainsSpace(
                "True True".to_string()
            ))
        );

        let query = "UPDATE key false";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update("key".to_string(), BuckTypes::Boolean(false)))
        );

        let query = "UPDATE key false True";
        let result = parse_query(query);
        assert_eq!(
            result,
            Err(BuckParserError::UpdateValueContainsSpace(
                "false True".to_string()
            ))
        );

        let query = "UPDATE key \"some string value\"";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_string(),
                BuckTypes::String("some string value".to_string())
            ))
        );

        let query = "UPDATE key \"FOO BAR BAZ\"";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_string(),
                BuckTypes::String("FOO BAR BAZ".to_string())
            ))
        );

        let query = "UPDATE key \"10 foo bar baz\"";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_string(),
                BuckTypes::String("10 foo bar baz".to_string())
            ))
        );

        let query = "UPDATE key \"foo bar baz\"";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_string(),
                BuckTypes::String("foo bar baz".to_string())
            ))
        );

        let query = "UPDATE key q1w2e3r4t5!!";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_string(),
                BuckTypes::Unknown("q1w2e3r4t5!!".to_string())
            ))
        );

        // hash query
        let query = "UPDATE key {key:1}";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_owned(),
                BuckTypes::Hash({
                    let mut h = HashMap::new();
                    h.insert("key".to_owned(), BuckTypes::Integer(1));
                    h
                })
            ))
        );

        let query = "UPDATE key {key1:1, key2:true, key3:1.0, key4:\"test\"}";
        let result = parse_query(query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_owned(),
                BuckTypes::Hash({
                    let mut h = HashMap::new();
                    h.insert("key1".to_owned(), BuckTypes::Integer(1));
                    h.insert("key2".to_owned(), BuckTypes::Boolean(true));
                    h.insert("key3".to_owned(), BuckTypes::Float(1.0));
                    h.insert("key4".to_owned(), BuckTypes::String("test".to_owned()));
                    h
                })
            ))
        );

        let sets_query = "UPDATE key (1, 2, 3, 4, 5)";
        let result = parse_query(sets_query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_owned(),
                BuckTypes::Sets(
                    vec!["1", "2", "3", "4", "5"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                )
            ))
        );

        let duplicated_sets_query = "UPDATE key (1, 2, 3, 4, 5, 1, 2, 3, 4, 5)";
        let result = parse_query(duplicated_sets_query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_owned(),
                BuckTypes::Sets(
                    vec!["1", "2", "3", "4", "5"]
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                )
            ))
        );

        let empty_sets_query = "UPDATE key ()";
        let result = parse_query(empty_sets_query);
        assert_eq!(
            result,
            Ok(Update(
                "key".to_owned(),
                BuckTypes::Sets(
                    vec![""]
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                )
            ))
        );

        ////// Error Cases //////

        let invalid_query = "UPDATE key";
        let result = parse_query(invalid_query);
        assert_eq!(
            result,
            Err(BuckParserError::InvalidQueryCommand(
                invalid_query.to_string()
            ))
        );

        let invalid_number_key = "UPDATE 1 2";
        let result = parse_query(invalid_number_key);
        assert_eq!(
            result,
            Err(BuckParserError::InvalidKey("1".to_string()))
        );

        let invalid_query = "UPDATE 1 2 3 4 5 6 7 8 9 10";
        let result = parse_query(invalid_query);
        assert_eq!(
            result,
            Err(BuckParserError::InvalidKey("1".to_string()))
        );

        let invalid_hash_query = "UPDATE key {key1:1, key2:}";
        let result = parse_query(invalid_hash_query);
        assert_eq!(
            result,
            Err(BuckParserError::HashValueIsEmpty("key2".to_owned()))
        );

        let invalid_hash_query = "UPDATE key {key1:1, :1}";
        let result = parse_query(invalid_hash_query);
        assert_eq!(
            result,
            Err(BuckParserError::HashKeyIsEmpty("1".to_owned()))
        )
    }

    #[test]
    fn test_parse_remove() {
        let query = "REMOVE key";
        let result = parse_query(query);
        assert_eq!(result, Ok(Remove(vec!["key".to_string()])));

        let multiple_keys = "REMOVE key1 key2 key3 key4 key5 key6 key7";
        let result = parse_query(multiple_keys);
        assert_eq!(
            result,
            Ok(Remove(vec![
                "key1".to_string(),
                "key2".to_string(),
                "key3".to_string(),
                "key4".to_string(),
                "key5".to_string(),
                "key6".to_string(),
                "key7".to_string(),
            ]))
        );

        let query = "REMOVE";
        let result = parse_query(query);
        assert_eq!(
            result,
            Err(BuckParserError::InvalidQueryCommand(query.to_string()))
        );

        let invalid_number_key = "REMOVE 1 2";
        let result = parse_query(invalid_number_key);
        assert_eq!(
            result,
            Err(BuckParserError::InvalidKey("1, 2".to_string()))
        );
    }
}
