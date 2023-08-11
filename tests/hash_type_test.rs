#[cfg(test)]
mod hash_method_tests {
    use std::collections::HashMap;

    use buck::{parser::{parse::parse_query, query::BuckQuery}, types::types::BuckTypes};

    #[test]
    fn test_parse_hset_single_fields() {
        let query = "hset integer key1:1";
        let result = parse_query(query);
        let expected = BuckQuery::HSet(
            "integer".to_string(),
            HashMap::from([("key1".to_string(), BuckTypes::Integer(1))]),
        );

        assert_eq!(result, Ok(expected));

        let query = "hset float key1:1.0";
        let result = parse_query(query);
        let expected = BuckQuery::HSet(
            "float".to_string(),
            HashMap::from([("key1".to_string(), BuckTypes::Float(1.0))]),
        );

        assert_eq!(result, Ok(expected));

        let query = "hset boolean key1:true";
        let result = parse_query(query);
        let expected = BuckQuery::HSet(
            "boolean".to_string(),
            HashMap::from([("key1".to_string(), BuckTypes::Boolean(true))]),
        );

        assert_eq!(result, Ok(expected));

        let query = "hset string key1:\"test\"";
        let result = parse_query(query);
        let expected = BuckQuery::HSet(
            "string".to_string(),
            HashMap::from([("key1".to_string(), BuckTypes::String("test".to_string()))]),
        );

        assert_eq!(result, Ok(expected));

        let query = "hset unknown key1:test";
        let result = parse_query(query);
        let expected = BuckQuery::HSet(
            "unknown".to_string(),
            HashMap::from([("key1".to_string(), BuckTypes::Unknown("test".to_string()))]),
        );

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_parse_hset_multiple_fields() {
        let query = "hset main key1:1 key2:value2 key3:true key4:\"test\"";
        let result = parse_query(query);
        let expected = BuckQuery::HSet(
            "main".to_string(),
            HashMap::from([
                ("key1".to_string(), BuckTypes::Integer(1)),
                ("key2".to_string(), BuckTypes::Unknown("value2".to_string())),
                ("key3".to_string(), BuckTypes::Boolean(true)),
                ("key4".to_string(), BuckTypes::String("test".to_string())),
            ]),
        );

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_value_contains_whitespace() {
        let query = "hset bike1 model:Deimos brand:Ergonom type:'Enduro bikes' price:4972";
        let result = parse_query(query);

        let expected = BuckQuery::HSet(
            "bike1".to_string(),
            HashMap::from([
                ("model".to_string(), BuckTypes::Unknown("Deimos".to_string())),
                ("brand".to_string(), BuckTypes::Unknown("Ergonom".to_string())),
                ("type".to_string(), BuckTypes::Unknown("Enduro bikes".to_string())),
                ("price".to_string(), BuckTypes::Integer(4972)),
            ]),
        );

        assert_eq!(result, Ok(expected));
    }
}