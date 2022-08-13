#[cfg(test)]
mod tests {
    use crate::json::{lexer::*, self, parser::{JsonNode, JsonValueOps}};

    const HAPPY_TEST: &str = r#"{
        "glossary": {
            "title": "example glossary",
            "GlossDiv": {
                "title": "S",
                "count": 5.123,
                "hours": -1
            }
        },
        "array": [1234567890123, -12.1, "S"]
        }"#;

    const SAD_TEST_UNTERMINATED_STR: &str = r#"{
        "glossary: {
            "title": "example glossary",
            "GlossDiv": {
                "title": "S",
                "count": 5.123,
                "hours": -1
            }
        },
        "array": [1234567890123, -12.1, "S"]
        }"#;

    const SAD_TEST_INVALID_FLT: &str = r#"{
            "glossary": {
                "title": "example glossary",
                "GlossDiv": {
                    "title": "S",
                    "count": 5123e-3,
                    "hours": -1
                }
            },
            "array": [1234567890123, -12.1, "S"]
        }"#;

    #[test] 
    fn test_parser_simple() {
        // First case: string!
        const STR_VALUE: &str = r#""test string""#;

        let mut lexer = JsonLexer::from_raw_json(STR_VALUE).unwrap();
        let json_file = json::parser::parse_json(&mut lexer);

        match json_file {
            Some(JsonNode::String(value)) => {
                assert_eq!(value.get(), "test string");
            },
            _ => {
                assert!(false);
            }
        }

        const NUMBER_VALUE: &str = r#"-100123"#;

        let mut lexer = JsonLexer::from_raw_json(NUMBER_VALUE).unwrap();
        let json_file = json::parser::parse_json(&mut lexer);

        match json_file {
            Some(JsonNode::Number(value)) => {
                assert_eq!(*value.get(), -100123);
            }
            _ => {
                assert!(false);
            }
        }

        const FLOAT_VALUE: &str = r#"-10.0123e2"#;

        let mut lexer = JsonLexer::from_raw_json(FLOAT_VALUE).unwrap();
        let json_file = json::parser::parse_json(&mut lexer);

        match json_file {
            Some(JsonNode::Float(value)) => {
                assert_eq!(*value.get(), -1001.23);
            }
            _ => {
                assert!(false);
            }
        }

        const ARRAY_VALUE: &str = r#"[1, 2, 3.12, 4, 5, 6]"#;
        let mut lexer = JsonLexer::from_raw_json(ARRAY_VALUE).unwrap();
        let json_file = json::parser::parse_json(&mut lexer);

        match json_file {
            Some(JsonNode::Float(value)) => {
                assert_eq!(*value.get(), -1001.23);
            }
            Some(JsonNode::Array(array)) => {
                for i in 0..array.size() {
                    let a = array.get(i).expect("fail?");

                    match a {
                        JsonNode::Object(_) => todo!(),
                        JsonNode::Array(_) => todo!(),
                        JsonNode::Number(num) => { 
                            println!("{}", *num.get());
                        }
                        JsonNode::Float(num) => {
                            println!("{}", *num.get());
                        },
                        JsonNode::Bool(_) => todo!(),
                        JsonNode::String(_) => todo!(),
                        JsonNode::Null => todo!(),
                    }
                }
            },
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_unterminated_str() {
        let mut lexer = JsonLexer::from_raw_json(SAD_TEST_UNTERMINATED_STR).unwrap();

        let mut token = Token::default();
        lexer.next_token(&mut token);

        let expected_token_types: Vec<TokenType> = vec![
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::Undefined
        ];
        let mut current_token = 0;

        while token.get_type() != TokenType::Undefined {
            assert_eq!(token.get_type(), expected_token_types[current_token]);
            current_token += 1;
            lexer.next_token(&mut token);
        }
    }

    #[test]
    fn test_invalid_flt() {
        let mut lexer = JsonLexer::from_raw_json(SAD_TEST_INVALID_FLT).unwrap();

        let mut token = Token::default();
        lexer.next_token(&mut token);

        let expected_token_types: Vec<TokenType> = vec![
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Float { value: 5.123 },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Number { value: -1 },
            TokenType::Reserve { reserve_id: ReserveCode::CloseBrace },
            TokenType::Reserve { reserve_id: ReserveCode::CloseBrace },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            // The array.
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBracket },
            TokenType::Number { value: 1234567890123 },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::Undefined
        ];
        let mut current_token = 0;

        while token.get_type() != TokenType::Undefined {
            assert_eq!(token.get_type(), expected_token_types[current_token]);
            current_token += 1;
            lexer.next_token(&mut token);
        }
    }

    #[test]
    // Load a json file from the system and see if we get the right
    // Token stream.
    fn test_happy_lexer() {
        let mut lexer = JsonLexer::from_raw_json(HAPPY_TEST).unwrap();

        let mut token = Token::default();
        lexer.next_token(&mut token);

        let expected_token_types: Vec<TokenType> = vec![
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Float { value: 5.123 },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Number { value: -1 },
            TokenType::Reserve { reserve_id: ReserveCode::CloseBrace },
            TokenType::Reserve { reserve_id: ReserveCode::CloseBrace },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            // The array.
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBracket },
            TokenType::Number { value: 1234567890123 },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::Float { value: -12.1 },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String,
            TokenType::Reserve { reserve_id: ReserveCode::CloseBracket },
            // End array.
            TokenType::Reserve { reserve_id: ReserveCode::CloseBrace },
        ];
        let mut current_token = 0;

        while token.get_type() != TokenType::Undefined {
            assert_eq!(token.get_type(), expected_token_types[current_token]);
            current_token += 1;
            lexer.next_token(&mut token);
        }
    }
}