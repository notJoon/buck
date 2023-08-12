#[cfg(test)]
mod encoding_tests {
    use buck::types::sets::EqFloat;

    #[test]
    fn test_boolean_encoding() {
        use buck::encoding::encoding::encode_boolean;

        assert_eq!(encode_boolean(true), 0x01);
        assert_eq!(encode_boolean(false), 0x00);
    }

    #[test]
    fn test_boolean_decoding() {
        use buck::encoding::encoding::decode_boolean;

        assert_eq!(decode_boolean(0x01), Ok(true));
        assert_eq!(decode_boolean(0x00), Ok(false));
        assert!(decode_boolean(0x02).is_err());
    }

    #[test]
    fn test_take_boolean() {
        use buck::encoding::encoding::take_boolean;

        let mut bytes = [0x01, 0x00, 0x02].as_ref();
        assert_eq!(take_boolean(&mut bytes), Ok(true));
        assert_eq!(take_boolean(&mut bytes), Ok(false));
        assert!(take_boolean(&mut bytes).is_err());
    }

    #[test]
    fn test_byte_encoding() {
        use buck::encoding::encoding::encode_bytes;

        assert_eq!(encode_bytes(&[0x00]), vec![0x00, 0xff, 0x00, 0x00]);
        assert_eq!(encode_bytes(&[0x01]), vec![0x01, 0x00, 0x00]);
        assert_eq!(
            encode_bytes(&[0x00, 0x01]),
            vec![0x00, 0xff, 0x01, 0x00, 0x00]
        );
        assert_eq!(
            encode_bytes(&[0x00, 0x00]),
            vec![0x00, 0xff, 0x00, 0xff, 0x00, 0x00]
        );
    }

    #[test]
    fn test_takes_bytes() {
        use buck::encoding::encoding::takes_bytes;

        let mut bytes: &[u8] = &[];
        assert!(takes_bytes(&mut bytes).is_err());

        let mut bytes: &[u8] = &[0x00, 0x00];
        assert_eq!(takes_bytes(&mut bytes), Ok(vec![]));

        let mut bytes: &[u8] = &[0x01, 0x02, 0x03, 0x00, 0x00, 0xa0, 0xb0];
        assert_eq!(takes_bytes(&mut bytes), Ok(vec![0x01, 0x02, 0x03]));
        assert_eq!(bytes, &[0xa0, 0xb0]);

        let mut bytes: &[u8] = &[0x00, 0xff, 0x01, 0x02, 0x00, 0x00];
        assert_eq!(takes_bytes(&mut bytes), Ok(vec![0x00, 0x01, 0x02]));
        assert!(bytes.is_empty());

        assert!(takes_bytes(&mut &[0x00][..]).is_err());
        assert!(takes_bytes(&mut &[0x01][..]).is_err());
        assert!(takes_bytes(&mut &[0x00, 0xff][..]).is_err());
        assert!(takes_bytes(&mut &[0x00, 0xff, 0x00][..]).is_err());
        assert!(takes_bytes(&mut &[0x00, 0x01, 0x00, 0x00][..]).is_err());
    }

    #[test]
    fn test_integer_encoding() {
        use buck::encoding::encoding::encode_integer;

        assert_eq!(encode_integer(std::i64::MIN), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(encode_integer(std::i64::MAX), [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(encode_integer(-1024), [0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc, 0x00]);
        assert_eq!(encode_integer(-42), [0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xd6]);
        assert_eq!(encode_integer(-1), [0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(encode_integer(0), [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(encode_integer(1), [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
        assert_eq!(encode_integer(42), [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a]);
        assert_eq!(encode_integer(1024), [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00]);
    }

    #[test]
    fn test_decode_integer() {
        use buck::encoding::encoding::decode_integer;

        assert_eq!(decode_integer([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), std::i64::MIN);
        assert_eq!(decode_integer([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]), std::i64::MAX);
        assert_eq!(decode_integer([0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc, 0x00]), -1024);
        assert_eq!(decode_integer([0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xd6]), -42);
        assert_eq!(decode_integer([0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]), -1);
        assert_eq!(decode_integer([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), 0);
        assert_eq!(decode_integer([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]), 1);
        assert_eq!(decode_integer([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a]), 42);
        assert_eq!(decode_integer([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00]), 1024);
    }

    #[test]
    fn test_take_integer() {
        use buck::encoding::encoding::take_integer;

        let mut bytes: &[u8] = &[];
        assert!(take_integer(&mut bytes).is_err());

        let mut bytes: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        assert!(take_integer(&mut bytes).is_err());
        assert_eq!(bytes.len(), 7);

        let mut bytes: &[u8] = &[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00];
        assert_eq!(take_integer(&mut bytes), Ok(1024));
        assert!(bytes.is_empty());

        let mut bytes: &[u8] = &[0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc, 0x00];
        assert_eq!(take_integer(&mut bytes), Ok(-1024));
        assert!(bytes.is_empty());
    }

    #[test]
    fn test_encode_float() {
        use buck::encoding::encoding::encode_float;
        use std::f64;
        use std::f64::consts::PI;

        assert_eq!(encode_float(f64::NEG_INFINITY), [0x00, 0x0f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(encode_float(-PI * 1e100), [0x2b, 0x33, 0x46, 0x0a, 0x3c, 0x0d, 0x14, 0x7b]);
        assert_eq!(encode_float(-PI * 1e2), [0x3f, 0x8c, 0x5d, 0x73, 0xa6, 0x2a, 0xbc, 0xc4]);
        assert_eq!(encode_float(-0_f64), [0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(encode_float(0_f64), [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(encode_float(PI), [0xc0, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18]);
        assert_eq!(encode_float(PI * 1e2), [0xc0, 0x73, 0xa2, 0x8c, 0x59, 0xd5, 0x43, 0x3b]);
        assert_eq!(encode_float(PI * 1e100), [0xd4, 0xcc, 0xb9, 0xf5, 0xc3, 0xf2, 0xeb, 0x84]);
        assert_eq!(encode_float(f64::INFINITY), [0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(encode_float(f64::NAN), [0xff, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn decode_float() {
        use buck::encoding::encoding::{decode_float, encode_float};
        use std::f64;
        use std::f64::consts::PI;

        assert_eq!(decode_float(encode_float(f64::NEG_INFINITY)), f64::NEG_INFINITY);
        assert_eq!(decode_float(encode_float(-PI)), -PI);
        assert_eq!(decode_float(encode_float(PI)), PI);
        assert_eq!(decode_float(encode_float(f64::INFINITY)), f64::INFINITY);
        assert!(decode_float(encode_float(f64::NAN)).is_nan());
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_take_float() {
        use buck::encoding::encoding::take_float;

        let mut bytes: &[u8] = &[];
        assert!(take_float::<f64>(&mut bytes).is_err());

        let mut bytes: &[u8] = &[0x01, 0x02, 0x03, 0x04];
        assert!(take_float::<f64>(&mut bytes).is_err());
        assert_eq!(bytes.len(), 4);

        let mut bytes: &[u8] = &[0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(take_float::<f64>(&mut bytes), Ok(0_f64));
        assert!(bytes.is_empty());

        let mut bytes: &[u8] = &[0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        assert_eq!(take_float::<f64>(&mut bytes), Ok(-0_f64));
        assert!(bytes.is_empty());

        let mut bytes: &[u8] = &[0xff, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(take_float::<f64>(&mut bytes), Ok(f64::INFINITY));
        assert!(bytes.is_empty());
    }

    #[test]
    fn test_encode_string() {
        use buck::encoding::encoding::encode_string;

        assert_eq!(encode_string(""), vec![0x00, 0x00]);
        assert_eq!(encode_string("a"), vec![0x61, 0x00, 0x00]);
        assert_eq!(encode_string("ab"), vec![0x61, 0x62, 0x00, 0x00]);
        assert_eq!(encode_string("abc"), vec![0x61, 0x62, 0x63, 0x00, 0x00]);
        assert_eq!(encode_string("abcd"), vec![0x61, 0x62, 0x63, 0x64, 0x00, 0x00]);
        assert_eq!(encode_string("some test string..", ), vec![
            0x73, 0x6f, 0x6d, 0x65, 
            0x20, 0x74, 0x65, 0x73, 
            0x74, 0x20, 0x73, 0x74, 
            0x72, 0x69, 0x6e, 0x67, 
            0x2e, 0x2e, 0x00, 0x00,
        ]);
        assert_eq!(encode_string("!@#$%6&*()<>?\":[]\\"), vec![
            0x21, 0x40, 0x23, 0x24, 
            0x25, 0x36, 0x26, 0x2a, 
            0x28, 0x29, 0x3c, 0x3e, 
            0x3f, 0x22, 0x3a, 0x5b, 
            0x5d, 0x5c, 0x00, 0x00,
        ]);
    }

    #[test]
    fn test_take_string() {
        use buck::encoding::encoding::take_string;

        let mut bytes: &[u8] = &[];
        assert!(take_string(&mut bytes).is_err());

        let mut bytes: &[u8] = &[0x00, 0x00];
        assert_eq!(take_string(&mut bytes), Ok("".to_owned()));

        let mut bytes: &[u8] = &[0x61, 0x00, 0x00];
        assert_eq!(take_string(&mut bytes), Ok("a".to_owned()));

        let mut bytes: &[u8] = &[0x61, 0x62, 0x00, 0x00];
        assert_eq!(take_string(&mut bytes), Ok("ab".to_owned()));

        let mut bytes: &[u8] = &[0x78, 0x20, 0x00, 0xff, 0x20, 0x7a, 0x00, 0x00, 0x01, 0x02, 0x03];
        assert_eq!(take_string(&mut bytes), Ok("x \u{0000} z".to_owned()));

        // invalid utf-8
        let mut bytes: &[u8] = &[0xff, 0x00, 0x00];
        assert!(take_string(&mut bytes).is_err());
    }

    #[test]
    fn test_encode_length() {
        use buck::encoding::encoding::encode_length;

        assert_eq!(encode_length(0), [0x00, 0x00, 0x00, 0x00]);
        assert_eq!(encode_length(1), [0x00, 0x00, 0x00, 0x01]);
        assert_eq!(encode_length(2), [0x00, 0x00, 0x00, 0x02]);
        assert_eq!(encode_length(3), [0x00, 0x00, 0x00, 0x03]);
        assert_eq!(encode_length(128), [0x00, 0x00, 0x00, 0x80]);
        assert_eq!(encode_length(256), [0x00, 0x00, 0x01, 0x00]);
        assert_eq!(encode_length(65536), [0x00, 0x01, 0x00, 0x00]);
        assert_eq!(encode_length(16777216), [0x01, 0x00, 0x00, 0x00]);
        assert_eq!(encode_length(4294967295), [0xff, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn test_encode_set() {
        use buck::types::sets::{BuckSets, Setable};
        use buck::encoding::encoding::encode_set;

        let mut set = BuckSets::new();
        set.data.insert(Setable::String("a".to_owned()));
    
        assert_eq!(encode_set(&set), vec![
            0x00, 0x00, 0x00, 0x01,     // length of the set
            0x01, 0x61, 0x00, 0x00,     // string type  `a`
        ]);

        let mut set = BuckSets::new();
        set.data.insert(Setable::Integer(123));

        assert_eq!(encode_set(&set), vec![
            0x00, 0x00, 0x00, 0x01,           // length of the set
            0x03, 0x80, 0x00, 0x00,           // integer type
            0x00, 0x00, 0x00, 0x00, 0x7b,     // integer value `123`
        ]);

        let mut set = BuckSets::new();
        set.data.insert(Setable::Boolean(true));

        assert_eq!(encode_set(&set), vec![
            0x00, 0x00, 0x00, 0x01,     // length of the set
            0x02, 0x01,                 // boolean type `true`
        ]);

        let mut set = BuckSets::new();
        set.data.insert(Setable::Boolean(false));

        assert_eq!(encode_set(&set), vec![
            0x00, 0x00, 0x00, 0x01,     // length of the set
            0x02, 0x00,                 // boolean type `false`
        ]);

        let mut set = BuckSets::new();
        set.data.insert(Setable::Float(EqFloat(1.0)));

        assert_eq!(encode_set(&set), vec![
            0x00, 0x00, 0x00, 0x01,         // length of the set
            0x04, 0xbf, 0xf0, 0x00,         // float type
            0x00, 0x00, 0x00, 0x00, 0x00,   // value `1.0`
        ]);

        // multiple values
        let mut set = BuckSets::new();
        set.data.insert(Setable::String("a".to_owned()));
        set.data.insert(Setable::String("b".to_owned()));
        set.data.insert(Setable::String("c".to_owned()));

        assert_eq!(encode_set(&set).len(), 16);
    }
}
