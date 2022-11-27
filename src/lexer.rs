use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    CloseBracket,
    OpenBracket,
    Comma,
    Equals,
    Number(f32),
    Text(String),
}

pub struct Lexer {
    input: String,
    next_token: Option<Token>,
    non_string_chars: HashSet<char>
}

impl Lexer {
    pub fn create(input: &str) -> Lexer {
        Lexer {
            input: String::from(input).chars().rev().collect(),
            next_token: None,
            non_string_chars: HashSet::from(['(', ')', '=', ','])
        }
    }

    pub fn next(&mut self) {
        self.next_token = self.consume();
        // println!("Next token: {:?}", self.next_token);
    }

    pub fn get_next_token(&self) -> Option<Token> {
        self.next_token.clone()
    }

    fn consume(&mut self) -> Option<Token> {
        while let Some(next_char) = self.input.pop() {
            if !next_char.is_whitespace() {
                self.input.push(next_char);
                break;
            }
        }

        if let Some(next_char) = self.input.pop() {
            if next_char == '(' {
                return Some(Token::OpenBracket);
            } else if next_char == ')' {
                return Some(Token::CloseBracket);
            } else if next_char == ',' {
                return Some(Token::Comma);
            } else if next_char == '=' {
                return Some(Token::Equals);
            } else if next_char.is_alphabetic() {
                let mut text_val = String::from(next_char);
                while let Some(next_char) = self.input.pop() {
                    if !self.non_string_chars.contains(&next_char) {
                        text_val.push(next_char);
                    } else {
                        // If the character is not part of the text token anymore put it back
                        self.input.push(next_char);
                        break;
                    }
                }
                // Because whitespace is allowed in the string we should trim off whitespace at the end
                return Some(Token::Text(text_val.trim_end().to_string()));
            } else if next_char.is_numeric() {
                // Parse number
                let mut num_val = String::from(next_char);
                while let Some(next_char) = self.input.pop() {
                    if next_char.is_alphanumeric() || next_char == '.' {
                        num_val.push(next_char);
                    } else {
                        // If the character is not part of the text token anymore put it back
                        self.input.push(next_char);
                        break;
                    }
                }
                let value: f32 = num_val.parse().expect("Could not parse f32 value!");
                return Some(Token::Number(value));
            } else {
                println!("Lexing error: {} is not alphabetic or numeric", next_char);
                return None;
            }
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::lexer::*;

    #[test]
    fn lex_number() {
        let input = "0123.034";
        let mut sut = Lexer::create(input);
        sut.next();
        let result = sut.get_next_token();
        assert_eq!(result, Some(Token::Number(123.034)));
    }

    #[test]
    fn lex_text() {
        let input = "This is a super nice str1ng 1234";
        let mut sut = Lexer::create(input);
        sut.next();
        let result = sut.get_next_token();
        assert_eq!(result, Some(Token::Text(String::from(input))));
    }

    #[test]
    fn lex_object() {
        let input = "MyClass(name=Hans, next_attribute=123)";
        let expected = vec![
            Token::Text(String::from("MyClass")),
            Token::OpenBracket,
            Token::Text(String::from("name")),
            Token::Equals,
            Token::Text(String::from("Hans")),
            Token::Comma,
            Token::Text(String::from("next_attribute")),
            Token::Equals,
            Token::Number(123.0),
            Token::CloseBracket
        ];
        let mut sut = Lexer::create(input);
        sut.next();
        let mut result = vec![];
        while let Some(token) = sut.get_next_token() {
            result.push(token);
            sut.next();
        }
        assert_eq!(result, expected);
    }
}
