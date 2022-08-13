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

    // TODO: create a json string to save to a file.
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
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JsonValue<T: PartialEq + Clone> {
    value: T
}

pub trait JsonValueOps<T: PartialEq + Clone> {
    fn set(&mut self, value: T);
    fn get_mut(&mut self) -> &mut T;
    fn get(&self) -> &T;
}

impl<T: PartialEq + Clone> JsonValueOps<T> for JsonValue<T> {
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
        TokenType::Reserve { reserve_id } => {
            match reserve_id {
                ReserveCode::OpenBrace => parse_object(lexer, token),
                ReserveCode::OpenBracket => parse_array(lexer, token),
                _ => None
            }
        },
        TokenType::Number { value } => Some(JsonNode::Number(JsonValue { value })),
        TokenType::Float { value } => Some(JsonNode::Float(JsonValue { value })),
        TokenType::Boolean { value } => Some(JsonNode::Bool(JsonValue { value })),
        TokenType::String => {
            // Get the actual string from the lexer.
            return match lexer.get_lexeme(token) {
                Some(value) => {
                    Some(JsonNode::String(JsonValue { value }))
                }
                // Error in lexer?
                None => None
            }
        },
        TokenType::Null => Some(JsonNode::Null),
        TokenType::Undefined => None
    }
}

fn parse_object(lexer: &mut JsonLexer, token: &mut Token) -> Option<JsonNode> {
    // We already know we are starting with an open brace, so just get the next token.
    lexer.next_token(token);
    None
}

fn parse_array(lexer: &mut JsonLexer, token: &mut Token) -> Option<JsonNode> {
    // We already know we are starting with an open bracket, so just get the next token.
    lexer.next_token(token);

    // Expect there to be a JsonNode and if there's a comma following it, expect another json node...
    let mut array = JsonArray::default();

    'load_array_values: loop {
        let node = parse_node(lexer, token);
        
        match node {
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

    Some(JsonNode::Array(array))
}
