#[cfg(test)]
mod query_tests {
    use buck::parser::parse::parse_query;
    use buck::parser::query::BuckQuery::Set;
    use buck::types::BuckTypes;

    #[test]
    fn test_parse_set() {
        let query = "SET key 1";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::Integer(1))));

        let query = "SET key 1_000_000_000_000";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::Integer(1000000000000))));

        let query = "SET key 1.0";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::Float(1.0))));

        let query = "SET key 1_000_000_000_000.012_345_678_900";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::Float(1000000000000.0123456789))));

        let query = "SET key true";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::Boolean(true))));

        let query = "SET key True";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::Unknown("True".to_string()))));

        let query = "SET key True True";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::Unknown("True True".to_string()))));

        let query = "SET key false";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::Boolean(false))));

        let query = "SET key \"test\"";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::String("test".to_string()))));

        let query = "SET key \"FOO BAR BAZ\"";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::String("FOO BAR BAZ".to_string()))));

        let query = "SET key \"foo bar baz\"";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::String("foo bar baz".to_string()))));

        let query = "SET key q1w2e3r4t5!!";
        let result = parse_query(query);
        assert_eq!(result, Ok(Set("key".to_string(), BuckTypes::Unknown("q1w2e3r4t5!!".to_string()))));
    } 
}