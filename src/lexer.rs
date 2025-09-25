// in src/lexer.rs

use crate::Token;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.source.len() {
            return Token::EndOfFile;
        }

        let current_char = self.source[self.position];

        let token = match current_char {
            '"' => self.read_text(),
            '=' => { // <-- ADDED
                self.position += 1;
                Token::Equals
            }
            _ if current_char.is_alphabetic() => self.read_identifier(),
            _ if current_char.is_digit(10) => self.read_number(), // <-- ADDED
            _ => Token::EndOfFile,
        };

        token
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.source.len() && self.source[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position].is_alphabetic() {
            self.position += 1;
        }
        let text: String = self.source[start..self.position].iter().collect();

        match text.as_str() {
            "say" => Token::Say,
            "create" => Token::Create, // <-- ADDED
            _ => Token::Identifier(text),
        }
    }

    // NEW FUNCTION to read numbers
    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position].is_digit(10) {
            self.position += 1;
        }
        let number_str: String = self.source[start..self.position].iter().collect();
        // We parse the string into a floating-point number (f64)
        Token::Number(number_str.parse().unwrap_or(0.0))
    }

    fn read_text(&mut self) -> Token {
        self.position += 1;
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position] != '"' {
            self.position += 1;
        }
        let text: String = self.source[start..self.position].iter().collect();
        self.position += 1;
        Token::Text(text)
    }
}