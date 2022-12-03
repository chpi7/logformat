use std::fmt;

use crate::ast::*;
use crate::lexer::{Lexer, Token};

/*
    MyClass(name = test123, other = 123, reference = MyOtherClass(id = 123, age = 23, name = heyho))

    Grammar:

    LogMessage := [Text] LogEntity [Text]
    LogEntity := Object | Text | Number
    Object := Text "(" [AttributeList] ")"          --> parse AttrList if next token is not ")"
    AttributeList := Attribute ("," Attribute)*
    Attribute := Text "=" [LogEntity]                 "name = test123" or "reference = Object (id = 123)" --> if next token is comma, no value exists
    Text := [a-zA-Z]+[a-zA-Z0-9]*
*/

pub struct Parser<'a> {
    lexer: Lexer,
    input: &'a str,
}

#[derive(Debug)]
pub struct ParseError {
    what: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError(what={})", self.what)
    }
}

impl Parser<'_> {
    pub fn create(input: &str) -> Parser {
        let lexer = Lexer::create(input);
        Parser { lexer, input }
    }

    pub fn parse_log_message(&mut self) -> Result<LogMessage, ParseError> {
        
        let mut bracket_counter = 0;
        let mut log_entites = vec![];
        let mut current_object_string = "".to_string();
        let mut current_log_string = "".to_string();

        for char in self.input.chars() {
            match char {
                ')' => {
                    current_object_string.push(char);
                    bracket_counter -= 1;
                    if bracket_counter == 0 {
                        // some object is probably fully done
                        self.lexer = Lexer::create(current_object_string.as_str());
                        self.lexer.next();
                        if let Ok(log_entity) = self.parse_log_entity() {
                            log_entites.push(log_entity);
                            current_log_string += format!(" {{{{ Log Entity {} }}}}", log_entites.len()).as_str();
                        } else {
                            current_log_string += " ";
                            current_log_string += current_object_string.as_str();
                        }
                        current_object_string = "".to_string();
                    }
                },
                '(' => {

                    if bracket_counter == 0 {
                        let message_parts: Vec<&str> = current_log_string.split(char::is_whitespace).collect();
                        if let Some((last, elements)) = message_parts.split_last() {
                            current_object_string += last;
                            current_log_string = elements.join(" ");
                        }
                    }

                    current_object_string.push(char);
                    bracket_counter += 1;
                },
                _ => {
                    if bracket_counter == 0 {
                        current_log_string.push(char);
                    } else {
                        current_object_string.push(char);
                    }
                }
            }
        }

        Ok(LogMessage { message: current_log_string + " " + current_object_string.as_str(), log_entity: log_entites })

    }

    pub fn parse_log_entity(&mut self) -> Result<LogEntity, ParseError> {
        if let Some(token) = self.lexer.get_next_token() {
            self.lexer.next();

            match token {
                Token::Number(value) => Ok(LogEntity::Number(value)),
                Token::Text(text) => {
                    if let Some(Token::OpenBracket) = self.lexer.get_next_token() {
                        self.lexer.next();

                        if let Some(Token::CloseBracket) = self.lexer.get_next_token() {
                            Ok(LogEntity::Object(
                                text,
                                AttributeList {
                                    attributes: Vec::new(),
                                },
                            ))
                        } else {
                            let attributes = self
                                .parse_attribute_list()
                                .expect("Could not parse expected attribute list.");
                            if let Some(Token::CloseBracket) = self.lexer.get_next_token() {
                                self.lexer.next();
                                Ok(LogEntity::Object(text, attributes))
                            } else {
                                Err(ParseError {
                                    what: format!(
                                    "Expected closing bracket after attribute list but found {:?}",
                                    self.lexer.get_next_token()
                                ),
                                })
                            }
                        }
                    } else {
                        Ok(LogEntity::Text(text))
                    }
                }
                _ => Err(ParseError {
                    what: format!("Unexpected token while parsing LogEntity: {:?}", token),
                }),
            }
        } else {
            Err(ParseError {
                what: String::from("No token remaining to parse LogEntity"),
            })
        }
    }

    fn parse_attribute_list(&mut self) -> Result<AttributeList, ParseError> {
        let first = self
            .parse_attribute()
            .expect("Expected attribute but could not parse one.");
        let mut attributes = vec![first];

        while let Some(Token::Comma) = self.lexer.get_next_token() {
            self.lexer.next();

            let next = self
                .parse_attribute()
                .expect("Expected attribute after comma, but could not parse one.");
            attributes.push(next);
        }
        // println!("Done parsing attribute list content");
        Ok(AttributeList { attributes })
    }

    fn parse_attribute(&mut self) -> Result<Attribute, ParseError> {
        // Parse name
        if let Some(Token::Text(attribute_name)) = self.lexer.get_next_token() {
            self.lexer.next();
            if let Some(Token::Equals) = self.lexer.get_next_token() {
                self.lexer.next();

                if let Some(Token::Comma) = self.lexer.get_next_token() {
                    // DONT consume the comma here because it is not part of the attribute!
                    return Ok(Attribute {
                        key: attribute_name,
                        value: LogEntity::Null,
                    });
                }

                let value = self
                    .parse_log_entity()
                    .expect("Expected LogEntity as attribute value but could not parse one.");
                Ok(Attribute {
                    key: attribute_name,
                    value,
                })
            } else {
                Err(ParseError {
                    what: format!(
                        "Expected equals between name and value but found {:?}",
                        self.lexer.get_next_token()
                    ),
                })
            }
        } else {
            Err(ParseError {
                what: format!(
                    "Expected attribute name but found token {:?}",
                    self.lexer.get_next_token()
                ),
            })
        }
    }
}
