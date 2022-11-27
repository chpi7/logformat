use crate::lexer::{Lexer, Token};
use crate::ast::*;

/*
    MyClass(name = test123, other = 123, reference = MyOtherClass(id = 123, age = 23, name = heyho))

    Grammar:

    LogEntity := Object | Text | Number
    Object := Text "(" [AttributeList] ")"          --> parse AttrList if next token is not ")"
    AttributeList := Attribute ("," Attribute)*
    Attribute := Text "=" LogEntity                 "name = test123" or "reference = Object (id = 123)"
    Text := [a-zA-Z]+[a-zA-Z0-9]*
*/

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn create(input: &str) -> Parser {
        let mut lexer = Lexer::create(input);
        lexer.next();
        Parser { lexer }
    }

    pub fn parse_log_entity(&mut self) -> Option<LogEntity> {
        if let Some(token) = self.lexer.get_next_token() {
            self.lexer.next();

            match token {
                Token::Number(value) => Some(LogEntity::Number(value)),
                Token::Text(text) => {
                    if let Some(Token::OpenBracket) = self.lexer.get_next_token() {
                        self.lexer.next();

                        if let Some(Token::CloseBracket) = self.lexer.get_next_token() {
                            Some(LogEntity::Object(
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
                                Some(LogEntity::Object(text, attributes))
                            } else {
                                println!("Expected closing bracket after attribute list but found {:?}", self.lexer.get_next_token());
                                None
                            }
                        }
                    } else {
                        Some(LogEntity::Text(text))
                    }
                }
                _ => {
                    println!("Unexpected token while parsing LogEntity: {:?}", token);
                    None
                }
            }
        } else {
            None
        }
    }

    fn parse_attribute_list(&mut self) -> Option<AttributeList> {
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
        Some(AttributeList { attributes })
    }

    fn parse_attribute(&mut self) -> Option<Attribute> {
        // Parse name
        if let Some(Token::Text(attribute_name)) = self.lexer.get_next_token() {
            self.lexer.next();
            if let Some(Token::Equals) = self.lexer.get_next_token() {
                self.lexer.next();
                let value = self
                    .parse_log_entity()
                    .expect("Expected LogEntity as attribute value but could not parse one.");
                Some(Attribute {
                    key: attribute_name,
                    value,
                })
            } else {
                println!(
                    "Expected equals between name and value but found {:?}",
                    self.lexer.get_next_token()
                );
                None
            }
        } else {
            println!(
                "Expected attribute name but found token {:?}",
                self.lexer.get_next_token()
            );
            None
        }
    }
}
