#[cfg(test)]
mod buck_type_tests {
    use std::collections::HashMap;

    use buck::parser::errors::BuckParserError;
    use buck::parser::parse::get_value_type;
    use buck::types::hash::BuckHash;
    use buck::types::sets::{BuckSets, Setable};
    use buck::types::types::{parse_hash, parse_sets, BuckTypes};

    #[test]
    fn test_type_inference() {
        assert_eq!(get_value_type("1"), Ok(BuckTypes::Integer(1)));
        assert_eq!(
            get_value_type("1_000_000_000_000"),
            Ok(BuckTypes::Integer(1000000000000))
        );
        assert_eq!(get_value_type("1.0"), Ok(BuckTypes::Float(1.0)));
        assert_eq!(get_value_type("true"), Ok(BuckTypes::Boolean(true)));
        assert_eq!(get_value_type("false"), Ok(BuckTypes::Boolean(false)));
        assert_eq!(
            get_value_type("\"test\""),
            Ok(BuckTypes::String("test".to_string()))
        );
        assert_eq!(
            get_value_type("\"test test test\""),
            Ok(BuckTypes::String("test test test".to_string()))
        );
        assert_eq!(
            get_value_type("test"),
            Ok(BuckTypes::Unknown("test".to_string()))
        );
    }

    #[test]
    fn test_parse_hash() {
        let input = "key1:1, key2:value2, key3:true, key4:\"test\"";
        let mut expected = BuckHash::new();
        expected.insert("key1".to_string(), BuckTypes::Integer(1));
        expected.insert("key2".to_string(), BuckTypes::Unknown("value2".to_string()));
        expected.insert("key3".to_string(), BuckTypes::Boolean(true));
        expected.insert("key4".to_string(), BuckTypes::String("test".to_string()));

        assert_eq!(parse_hash(input), Ok(expected));

        let empty_key = ":value1";
        assert_eq!(
            parse_hash(empty_key),
            Err(BuckParserError::HashKeyIsEmpty("value1".to_string()))
        );

        let empty_value = "key1:";
        assert_eq!(
            parse_hash(empty_value),
            Err(BuckParserError::HashValueIsEmpty("key1".to_string()))
        );

        let hash_input = "k1:v1, k2:v2, k3:, k4:v4";
        assert_eq!(
            parse_hash(hash_input),
            Err(BuckParserError::HashValueIsEmpty("k3".to_string()))
        );
    }
}
