#[derive(Debug, Clone)]
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
    next_token: Option<Token>
}

impl Lexer {
    pub fn create(input: &str) -> Lexer {
        Lexer { 
            input: String::from(input).chars().rev().collect(),
            next_token: None
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
                    if next_char.is_alphanumeric() {
                        text_val.push(next_char);
                    } else {
                        // If the character is not part of the text token anymore put it back
                        self.input.push(next_char);
                        break;
                    }
                }
                return Some(Token::Text(text_val));
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