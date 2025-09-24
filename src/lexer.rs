// First, we need to bring our Token definition into this file's scope.
use crate::Token;

// The Lexer holds the source code and our position in it.
pub struct Lexer {
    source: Vec<char>, // The source code as a list of characters
    position: usize,   // Our current position in the source code
}

impl Lexer {
    // A function to create a new Lexer from a string of code.
    pub fn new(source: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            position: 0,
        }
    }

    // This is the most important function. It reads the source code
    // and returns the very next token it finds.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // Check if we're at the end of the file.
        if self.position >= self.source.len() {
            return Token::EndOfFile;
        }

        let current_char = self.source[self.position];

        let token = match current_char {
            // If we see a double quote, we know we have a string literal.
            '"' => self.read_text(),
            // Anything else for now, we'll try to read it as a word (identifier or keyword).
            _ if current_char.is_alphabetic() => self.read_identifier(),

            // If we don't recognize the character, we'll have a placeholder.
            // We will add more characters here later (like numbers and operators).
            _ => Token::EndOfFile, // Placeholder for now
        };

        token
    }

    // A helper function to advance our position past any spaces or newlines.
    fn skip_whitespace(&mut self) {
        while self.position < self.source.len() && self.source[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    // A helper function for reading a word (like "say" or a variable name).
    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position].is_alphabetic() {
            self.position += 1;
        }
        let text: String = self.source[start..self.position].iter().collect();

        // Check if the word is a keyword, otherwise it's a variable name (Identifier).
        match text.as_str() {
            "say" => Token::Say,
            // We'll add "create", "if", etc. here later.
            _ => Token::Identifier(text),
        }
    }

    // A helper function for reading a string literal inside double quotes.
    fn read_text(&mut self) -> Token {
        self.position += 1; // Move past the opening "
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position] != '"' {
            self.position += 1;
        }
        let text: String = self.source[start..self.position].iter().collect();
        self.position += 1; // Move past the closing "
        Token::Text(text)
    }
}