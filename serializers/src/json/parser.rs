use std::{collections::{HashMap, hash_map::Keys}, str::FromStr};

use crate::json::lexer::*;

use super::lexer;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct JsonObject {
    sub_nodes: HashMap<String, JsonNode>
}

impl JsonObject {
    pub fn new() -> JsonObject {
        JsonObject { sub_nodes: HashMap::new() }
    }

    pub fn add(&mut self, name: &str, node: JsonNode) {
        self.sub_nodes.insert(String::from_str(name).unwrap(), node);
    }

    pub fn remove(&mut self, name: &str) -> Option<JsonNode> {
        self.sub_nodes.remove(name)
    }

    pub fn get(&self, name: &str) -> Option<&JsonNode> {
        self.sub_nodes.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut JsonNode> {
        self.sub_nodes.get_mut(name)
    }

    /// Returns the total number of elements.
    pub fn size(&self) -> usize {
        self.sub_nodes.len()
    }
    
    /// Returns if the key exists in the object.
    pub fn contains_key(&self, name: &str) -> bool {
        self.sub_nodes.contains_key(name)
    }

    /// Returns an iterator through all keys in the object.
    pub fn keys(&self) -> Keys<'_, String, JsonNode> {
        self.sub_nodes.keys()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, JsonNode> {
        self.sub_nodes.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, JsonNode> {
        self.sub_nodes.iter_mut()
    }
}

impl ToString for JsonObject {
    fn to_string(&self) -> String {
        if self.sub_nodes.len() == 0 { return "{}". to_string() }

        let mut returned_string = "{".to_string();

        let mut node_ittr = self.sub_nodes.iter();
        let mut current_itt = node_ittr.next().unwrap();

        loop {
            returned_string.push_str("\"");
            returned_string.push_str(current_itt.0);
            returned_string.push_str("\"");
            returned_string.push_str(":");
            returned_string.push_str(current_itt.1.to_string().as_str());

            let next_itt = node_ittr.next();

            match next_itt {
                None => break,
                Some(next) => {
                    returned_string.push_str(",");
                    current_itt = next;
                }
            }
        }

        returned_string.push_str("}");

        returned_string
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct JsonArray {
    sub_nodes: Vec<JsonNode>
}

impl JsonArray {
    pub fn new() -> JsonArray {
        JsonArray { sub_nodes: Vec::<JsonNode>::new() }
    }

    pub fn add(&mut self, node: JsonNode) {
        self.sub_nodes.push(node);
    }

    pub fn get(&self, index: usize) -> Option<&JsonNode> {
        self.sub_nodes.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut JsonNode> {
        self.sub_nodes.get_mut(index)
    }

    /// Removes an item from the array and returns it if it exists.
    pub fn remove(&mut self, index: usize) -> Option<JsonNode> { 
        return if index < self.size() {
            Some(self.sub_nodes.remove(index))
        }
        else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.sub_nodes.len()
    }

    pub fn iter(&self) -> std::slice::Iter<JsonNode> {
        self.sub_nodes.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<JsonNode> {
        self.sub_nodes.iter_mut()
    }
}

impl ToString for JsonArray {
    fn to_string(&self) -> String {
        if self.sub_nodes.len() == 0 { return "[]". to_string() }

        let mut returned_string = "[".to_string();

        let mut node_ittr = self.sub_nodes.iter();
        let mut current_itt = node_ittr.next().unwrap();

        loop {
            returned_string.push_str(current_itt.to_string().as_str());

            let next_itt = node_ittr.next();

            match next_itt {
                None => break,
                Some(next) => {
                    returned_string.push_str(",");
                    current_itt = next;
                }
            }
        }

        returned_string.push_str("]");

        returned_string
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JsonValue<T: PartialEq + Clone> {
    value: T
}

impl<T: PartialEq + Clone> JsonValue<T> {
    pub fn new(value: T) -> JsonValue<T> {
        JsonValue {
            value
        }
    }
}


pub trait JsonValueOps<T: PartialEq + Clone + ToString> {
    fn set(&mut self, value: T);
    fn get_mut(&mut self) -> &mut T;
    fn get(&self) -> &T;
}

impl<T: PartialEq + Clone + ToString> ToString for JsonValue<T> {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl<T: PartialEq + Clone + ToString> JsonValueOps<T> for JsonValue<T> {
    fn set(&mut self, value: T) {
        self.value = value;
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    fn get(&self) -> &T {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum JsonNode {
    Object(JsonObject),
    Array(JsonArray),
    Number(JsonValue<i64>),
    Float(JsonValue<f64>),
    Bool(JsonValue<bool>),
    String(JsonValue<String>),
    #[default]
    Null
}

impl ToString for JsonNode {
    fn to_string(&self) -> String {
        match self {
            JsonNode::Object(obj) => obj.to_string(),
            JsonNode::Array(arr) => arr.to_string(),
            JsonNode::Number(num) => num.to_string(),
            JsonNode::Float(num) => num.to_string(),
            JsonNode::Bool(val) => val.to_string(),
            JsonNode::String(str) => {
                let mut as_string = "\"".to_string();
                as_string.push_str(str.to_string().as_str());
                as_string.push_str("\"");
                as_string
            },
            JsonNode::Null => "null".to_string(),
        }
    }
}

pub fn parse_json(lexer: &mut JsonLexer) -> Option<JsonNode> {
    let mut token = Token::default();
    lexer.reset();
    lexer.next_token(&mut token);

    // Execute the CFG.
    parse_node(lexer, &mut token)
}

fn parse_node(lexer: &mut JsonLexer, token: &mut Token) -> Option<JsonNode> {
    // At this context, we are expecting anything except for syntax tokens.
    return match token.get_type() {
        TokenType::Reserve { reserve_id: lexer::ReserveCode::OpenBrace } => parse_object(lexer, token),
        TokenType::Reserve { reserve_id: lexer::ReserveCode::OpenBracket } => parse_array(lexer, token),
        TokenType::Number { value } => Some(JsonNode::Number(JsonValue { value })),
        TokenType::Float { value } => Some(JsonNode::Float(JsonValue { value })),
        TokenType::Boolean { value } => Some(JsonNode::Bool(JsonValue { value })),
        TokenType::String { value } => Some(JsonNode::String(JsonValue { value })),
        TokenType::Null => Some(JsonNode::Null),
        TokenType::Undefined => None,
        _ => {
            println!("Expected a json node");
            None
        }
    }
}

fn parse_object(lexer: &mut JsonLexer, token: &mut Token) -> Option<JsonNode> {
    // We already know we are starting with an open brace, so just get the next token.
    lexer.next_token(token);

    // Objects are formatted as follows: key: string: node: "json_node" 
    // Then they can have a comma and another string: node, so on.
    let mut obj = JsonObject::default();

    // Handle empty object case.
    if let TokenType::Reserve { reserve_id: lexer::ReserveCode::CloseBrace } = token.get_type() {
        return Some(JsonNode::Object(obj));
    }

    'find_object: loop {
        match token.get_type() {
            TokenType::String { value } => {
                lexer.next_token(token);

                if let TokenType::Reserve { reserve_id: lexer::ReserveCode::Colon } = token.get_type() {
                    lexer.next_token(token);

                    // Load the next node.
                    match parse_node(lexer, token) {
                        Some(loaded_node) => {
                            obj.add(value.as_str(), loaded_node);
                            lexer.next_token(token);
                        },
                        None => return None
                    }
                }
                else {
                    println!("Expected ':'");
                    return None;
                }
            },
            _ => {
                println!("Expected an object key");
                return None;
            }
        }

        // If there's no comma, we are done finding objects.
        if let TokenType::Reserve { reserve_id: lexer::ReserveCode::Comma } = token.get_type() {
            lexer.next_token(token);
        }
        else {
            break 'find_object;
        }
    }

    return if let TokenType::Reserve { reserve_id: lexer::ReserveCode::CloseBrace } = token.get_type() {
        Some(JsonNode::Object(obj))
    }
    else {
        println!("Expected '}}'");
        None
    }
}

fn parse_array(lexer: &mut JsonLexer, token: &mut Token) -> Option<JsonNode> {
    // We already know we are starting with an open bracket, so just get the next token.
    lexer.next_token(token);

    // Expect there to be a JsonNode and if there's a comma following it, expect another json node...
    let mut array = JsonArray::default();

    // Handle empty array case.
    if let TokenType::Reserve { reserve_id: lexer::ReserveCode::CloseBracket } = token.get_type() {
        return Some(JsonNode::Array(array))
    }

    'load_array_values: loop {
        match parse_node(lexer, token) {
            Some(node) => {
                array.add(node);
                lexer.next_token(token);
            },
            // If an error has been found, just break out.
            None => return None
        }

        if let TokenType::Reserve { reserve_id: lexer::ReserveCode::Comma } = token.get_type() {
            lexer.next_token(token);
        }
        else {
            break 'load_array_values;
        }
    }

    return if let TokenType::Reserve { reserve_id: lexer::ReserveCode::CloseBracket } = token.get_type() {
        Some(JsonNode::Array(array))
    }
    else {
        println!("Expected ']'");
        None
    }
}
