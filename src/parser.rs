use crate::ast::*;
use crate::lexer::{Lexer, Token};

/*
    MyClass(name = test123, other = 123, reference = MyOtherClass(id = 123, age = 23, name = heyho))

    Grammar:

    LogEntity := Object | Text | Number
    Object := Text "(" [AttributeList] ")"          --> parse AttrList if next token is not ")"
    AttributeList := Attribute ("," Attribute)*
    Attribute := Text "=" [LogEntity]                 "name = test123" or "reference = Object (id = 123)" --> if next token is comma, no value exists
    Text := [a-zA-Z]+[a-zA-Z0-9]*
*/

pub struct Parser {
    lexer: Lexer,
}

#[derive(Debug)]
pub struct ParseError {
    what: String,
}

impl Parser {
    pub fn create(input: &str) -> Parser {
        let mut lexer = Lexer::create(input);
        lexer.next();
        Parser { lexer }
    }

    pub fn parse_log_message(&mut self) -> Result<LogMessage, ParseError> {
        if let Some(Token::Text(text)) = self.lexer.get_next_token() {
            let parts: Vec<&str> = text.split(char::is_whitespace).collect();
            let mut class_name = String::from("");
            let mut message_before = text.clone();

            if let Some((last, remainder)) = parts.split_last() {
                class_name = String::from(*last);
                message_before = (*remainder).join(" ");
            }

            // Make it appear to the parser as if this step never happened
            // self.lexer.next();
            self.lexer.overwrite_token(Token::Text(class_name));

            let log_entity = self.parse_log_entity()?;
            let remaining_text = if let Some(Token::Text(t)) = self.lexer.get_next_token() {
                t
            } else {
                String::from("")
            };
            return Ok(LogMessage {
                message: message_before + " {{ log entity }} " + remaining_text.as_str(),
                log_entity,
            });
        } else {
            return Err(ParseError {
                what: format!(
                    "Expected LogMessage to start with a string but found {:?}",
                    self.lexer.get_next_token()
                ),
            });
        }
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
