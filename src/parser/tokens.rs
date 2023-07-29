// TODO : Change hard-coded tokens to BuckToken enum
#[derive(Debug, Eq, PartialEq)]
pub enum BuckToken {
    RangeDelimiter,         // ..
    ValueSeparator,         // ' '
    HashKeySeparator,       // :
    Underscore,             // _
    Comma,                  // ,
    Dot,                    // .
    OpenBracket,            // [
    CloseBracket,           // ]
    OpenParenthesis,        // (
    CloseParenthesis,       // )
    OpenBrace,              // {
    CloseBrace,             // }
    Value(String),
}

pub fn tokenize(input: &str) -> Vec<BuckToken> {
    let mut tokens = Vec::new();

    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        let token = match ch {
            '.' if chars.peek() == Some(&'.') => {
                chars.next();
                BuckToken::RangeDelimiter
            },
            ' ' => BuckToken::ValueSeparator,
            ':' => BuckToken::HashKeySeparator,
            '_' => BuckToken::Underscore,
            ',' => BuckToken::Comma,
            '.' => BuckToken::Dot,
            '[' => BuckToken::OpenBracket,
            ']' => BuckToken::CloseBracket,
            '(' => BuckToken::OpenParenthesis,
            ')' => BuckToken::CloseParenthesis,
            '{' => BuckToken::OpenBrace,
            '}' => BuckToken::CloseBrace,
            _ => {
                let mut value = ch.to_string();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_whitespace() || next_ch == ':' || next_ch == '_' || next_ch == ',' || next_ch == '.' || next_ch == '[' || next_ch == ']' || next_ch == '(' || next_ch == ')' || next_ch == '{' || next_ch == '}' {
                        break;
                    }
                    value.push(chars.next().unwrap());
                }

                BuckToken::Value(value)
            }
        };

        tokens.push(token);
    }

    tokens
}