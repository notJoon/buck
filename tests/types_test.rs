#[cfg(test)]
mod buck_type_tests {
    use buck::parser::parse::get_value_type;
    use buck::types::BuckTypes;

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
}
