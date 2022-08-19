#[cfg(test)]
mod tests {

    use crate::json::{lexer::{*}, self, parser::{JsonNode, JsonValueOps, JsonArray, JsonValue, JsonObject, parse_json}};

    const HAPPY_TEST: &str = r#"{
        "glossary": {
            "title": "example glossary",
            "GlossDiv": {
                "title": "S",
                "count": 5.123,
                "hours": -1,
                "is_true": false,
                "test_null": null
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
    /// Just a simple test to verify that empty objects are working as expected. 
    /// They should parse, but there should just be nothing inside them.
    fn test_with_empty_objets() {
        const EMPTY_ARRAY: &str = r#"[]"#;
        const EMPTY_OBJECT: &str = r#"{}"#;

        let mut lexer = JsonLexer::from_raw_json(EMPTY_ARRAY).unwrap();
        let json_file = json::parser::parse_json(&mut lexer);
        
        let expected_array = JsonArray::new();
        let expected_object = JsonObject::new();

        match json_file {
            Some(JsonNode::Array(array)) => {
                assert!(array.eq(&expected_array));
            }
            _ => assert!(false)
        }

        let mut lexer = JsonLexer::from_raw_json(EMPTY_OBJECT).unwrap();
        let json_file = json::parser::parse_json(&mut lexer);

        match json_file {
            Some(JsonNode::Object(object)) => {
                assert!(object.eq(&expected_object));
            }
            _ => assert!(false)
        }
    }

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

        const ARRAY_VALUE: &str = r#"[1, 2, 3.12, 4, 5, 6, ["test", 12]]"#;
        let mut lexer = JsonLexer::from_raw_json(ARRAY_VALUE).unwrap();
        let json_file = json::parser::parse_json(&mut lexer);

        let mut expected_array = JsonArray::default();
        expected_array.add(JsonNode::Number(JsonValue::<i64>::new(1)));
        expected_array.add(JsonNode::Number(JsonValue::<i64>::new(2)));
        expected_array.add(JsonNode::Float(JsonValue::<f64>::new(3.12)));
        expected_array.add(JsonNode::Number(JsonValue::<i64>::new(4)));
        expected_array.add(JsonNode::Number(JsonValue::<i64>::new(5)));
        expected_array.add(JsonNode::Number(JsonValue::<i64>::new(6)));

        let mut expected_sub_arr = JsonArray::default();
        expected_sub_arr.add(JsonNode::String(JsonValue::<String>::new("test".to_string())));
        expected_sub_arr.add(JsonNode::Number(JsonValue::<i64>::new(12)));
        expected_array.add(JsonNode::Array(expected_sub_arr));

        assert!(match json_file {
            Some(JsonNode::Array(array)) => {
                array.eq(&expected_array)

            },
            _ => {
                false
            }
        });

        // Load the happy test and see if we parse it correctly.
        let mut lexer = JsonLexer::from_raw_json(HAPPY_TEST).unwrap();
        let json_file = json::parser::parse_json(&mut lexer);

        let mut expected_object = JsonObject::default();
        let mut glossary_object = JsonObject::default();
        let mut glossary_div_object = JsonObject::default();

        glossary_object.add("title", JsonNode::String(JsonValue::<String>::new("example glossary".to_string())));
        glossary_div_object.add("title", JsonNode::String(JsonValue::<String>::new("S".to_string())));
        glossary_div_object.add("count", JsonNode::Float(JsonValue::<f64>::new(5.123)));
        glossary_div_object.add("hours", JsonNode::Number(JsonValue::<i64>::new(-1)));
        glossary_div_object.add("is_true", JsonNode::Bool(JsonValue::<bool>::new(false)));
        glossary_div_object.add("test_null", JsonNode::Null);
        glossary_object.add("GlossDiv", JsonNode::Object(glossary_div_object));
        expected_object.add("glossary", JsonNode::Object(glossary_object));

        let mut expected_array = JsonArray::default();
        expected_array.add(JsonNode::Number(JsonValue::<i64>::new(1234567890123)));
        expected_array.add(JsonNode::Float(JsonValue::<f64>::new(-12.1)));
        expected_array.add(JsonNode::String(JsonValue::<String>::new("S".to_string())));
        expected_object.add("array", JsonNode::Array(expected_array));

        match json_file {
            Some(JsonNode::Object(json_node)) => {
                assert!(json_node.eq(&expected_object));
            }
            _ => assert!(false)
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
            TokenType::String { value: "glossary".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String { value: "title".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::String { value: "example glossary".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String { value: "GlossDiv".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String { value: "title".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::String { value: "S".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String { value: "count".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Number { value: 5123 },
        ];
        let mut current_token = 0;

        while token.get_type() != TokenType::Undefined {
            assert_eq!(token.get_type(), expected_token_types[current_token]);
            current_token += 1;
            lexer.next_token(&mut token);
        }

        assert_eq!(current_token, expected_token_types.len());
    }

    #[test]
    fn test_json_to_string() {
        // Load the happy test and see if we parse it correctly.
        let mut expected_object = JsonObject::default();
        let mut glossary_object = JsonObject::default();
        let mut glossary_div_object = JsonObject::default();

        glossary_object.add("title", JsonNode::String(JsonValue::<String>::new("example glossary".to_string())));
        glossary_div_object.add("title", JsonNode::String(JsonValue::<String>::new("S".to_string())));
        glossary_div_object.add("count", JsonNode::Float(JsonValue::<f64>::new(5.123)));
        glossary_div_object.add("hours", JsonNode::Number(JsonValue::<i64>::new(-1)));
        glossary_div_object.add("is_true", JsonNode::Bool(JsonValue::<bool>::new(false)));
        glossary_div_object.add("test_null", JsonNode::Null);
        glossary_object.add("GlossDiv", JsonNode::Object(glossary_div_object));
        expected_object.add("glossary", JsonNode::Object(glossary_object));

        let mut expected_array = JsonArray::default();
        expected_array.add(JsonNode::Number(JsonValue::<i64>::new(1234567890123)));
        expected_array.add(JsonNode::Float(JsonValue::<f64>::new(-12.1)));
        expected_array.add(JsonNode::String(JsonValue::<String>::new("S".to_string())));
        expected_object.add("array", JsonNode::Array(expected_array));

        let string = expected_object.to_string();

        // Reparse the string to json, and see if it matches the source.
        let mut lexer = JsonLexer::from_raw_json(string.as_str()).unwrap();
        let loaded_json = parse_json(&mut lexer);

        match loaded_json {
            Some(JsonNode::Object(json_object)) => {
                assert!(json_object.eq(&expected_object))
            },
            _ => assert!(false)
        }
    }

    // Load a json file from the system and see if we get the right
    // Token stream.
    #[test]
    fn test_happy_lexer() {
        let mut lexer = JsonLexer::from_raw_json(HAPPY_TEST).unwrap();

        let mut token = Token::default();
        lexer.next_token(&mut token);

        let expected_token_types: Vec<TokenType> = vec![
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String { value: "glossary".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String { value: "title".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::String { value: "example glossary".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String { value: "GlossDiv".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBrace },
            TokenType::String { value: "title".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::String { value: "S".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String { value: "count".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Float { value: 5.123 },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String { value: "hours".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Number { value: -1 },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            // Boolean and null tests
            TokenType::String { value: "is_true".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Boolean { value: false },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String { value: "test_null".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Null,
            // End of bool and null tests
            TokenType::Reserve { reserve_id: ReserveCode::CloseBrace },
            TokenType::Reserve { reserve_id: ReserveCode::CloseBrace },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            // The array.
            TokenType::String { value: "array".to_string() },
            TokenType::Reserve { reserve_id: ReserveCode::Colon },
            TokenType::Reserve { reserve_id: ReserveCode::OpenBracket },
            TokenType::Number { value: 1234567890123 },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::Float { value: -12.1 },
            TokenType::Reserve { reserve_id: ReserveCode::Comma },
            TokenType::String {value: "S".to_string() },
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

        assert_eq!(current_token, expected_token_types.len());
    }
}