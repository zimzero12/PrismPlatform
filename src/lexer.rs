// in src/lexer.rs

use crate::token::Token;

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
        // This loop is the core of the fix.
        // It will continue to run until it finds a real token or hits the end of the file.
        loop {
            // First, skip any whitespace (spaces, newlines, etc.)
            self.skip_whitespace();

            // Check if we're at the end of the source code.
            if self.position >= self.source.len() {
                return Token::EndOfFile;
            }

            // Check for comments. If we find one, skip it and loop again.
            if self.source[self.position] == '#' {
                self.skip_comment();
                continue; // Go back to the start of the loop
            }

            // If it's not whitespace and not a comment, it must be a real token.
            // Break the loop and proceed to the token parsing logic below.
            break;
        }

        let current_char = self.source[self.position];

        // This token parsing logic remains the same.
        let token = match current_char {
            '=' => {
                self.position += 1;
                Token::Equals
            }
            '"' => self.read_text(),
            _ if current_char.is_alphabetic() => self.read_identifier(),
            _ if current_char.is_digit(10) => self.read_number(),
            _ => Token::EndOfFile,
        };

        token
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.source.len() && self.source[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    // This function now skips until it finds a newline character.
    fn skip_comment(&mut self) {
        while self.position < self.source.len() && self.source[self.position] != '\n' {
            self.position += 1;
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position].is_alphanumeric() {
            self.position += 1;
        }
        let text: String = self.source[start..self.position].iter().collect();

        match text.as_str() {
            "say" => Token::Say,
            "create" => Token::Create,
            _ => Token::Identifier(text),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position].is_digit(10) {
            self.position += 1;
        }
        let number_str: String = self.source[start..self.position].iter().collect();
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