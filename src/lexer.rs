use crate::error::{ParseError, TraceResult};
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

    pub fn next_token(&mut self) -> TraceResult<Token> {
        self.skip_whitespace();

        if self.position >= self.source.len() {
            return Ok(Token::EndOfFile);
        }

        let current_char = self.source[self.position];

        match current_char {
            '=' => {
                self.position += 1;
                Ok(Token::Equals)
            }
            '"' => Ok(self.read_text()),
            '#' => {
                self.skip_comment();
                self.next_token()
            }
            _ if current_char.is_alphabetic() => Ok(self.read_identifier()),
            _ if current_char.is_digit(10) => self.read_number(),
            _ => {
                self.position += 1;
                Ok(Token::EndOfFile)
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.source.len() && self.source[self.position].is_whitespace() {
            self.position += 1;
        }
    }

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

    // THIS IS THE CORRECTED FUNCTION
    fn read_number(&mut self) -> TraceResult<Token> {
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position].is_digit(10) {
            self.position += 1;
        }

        // THE FIX: After reading digits, we peek at the next character.
        if self.position < self.source.len() && self.source[self.position].is_alphabetic() {
            // If it's a letter, this is an invalid token like '123abc'.
            // We consume the rest of the invalid word to prevent partial reads.
            while self.position < self.source.len() && self.source[self.position].is_alphanumeric() {
                self.position += 1;
            }
            let invalid_str: String = self.source[start..self.position].iter().collect();
            // And we return the error the test expects.
            return Err(Box::new(ParseError::InvalidNumber(invalid_str)));
        }

        // If the character after the digits is not a letter, it's a valid number.
        let number_str: String = self.source[start..self.position].iter().collect();
        match number_str.parse::<f64>() {
            Ok(num) => Ok(Token::Number(num)),
            Err(_) => Err(Box::new(ParseError::InvalidNumber(number_str))),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_create_statement() -> TraceResult<()> {
        let source = "create score = 100";
        let mut lexer = Lexer::new(source.to_string());

        let expected_tokens = vec![
            Token::Create,
            Token::Identifier("score".to_string()),
            Token::Equals,
            Token::Number(100.0),
            Token::EndOfFile,
        ];

        for expected in expected_tokens {
            assert_eq!(lexer.next_token()?, expected);
        }
        Ok(())
    }

    #[test]
    fn test_say_statement_with_text() -> TraceResult<()> {
        let source = "say \"Hello World\"";
        let mut lexer = Lexer::new(source.to_string());

        let expected_tokens = vec![
            Token::Say,
            Token::Text("Hello World".to_string()),
            Token::EndOfFile,
        ];

        for expected in expected_tokens {
            assert_eq!(lexer.next_token()?, expected);
        }
        Ok(())
    }

    #[test]
    fn test_comments_are_skipped() -> TraceResult<()> {
        let source = "# This is a comment\ncreate x = 10 # another comment";
        let mut lexer = Lexer::new(source.to_string());

        let expected_tokens = vec![
            Token::Create,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::Number(10.0),
            Token::EndOfFile,
        ];

        for expected in expected_tokens {
            assert_eq!(lexer.next_token()?, expected);
        }
        Ok(())
    }
    
    #[test]
    fn test_whitespace_handling() -> TraceResult<()> {
        let source = "  create\tname\n=\r\n\"Zim\"  ";
        let mut lexer = Lexer::new(source.to_string());

        let expected_tokens = vec![
            Token::Create,
            Token::Identifier("name".to_string()),
            Token::Equals,
            Token::Text("Zim".to_string()),
            Token::EndOfFile,
        ];

        for expected in expected_tokens {
            assert_eq!(lexer.next_token()?, expected);
        }
        Ok(())
    }

    #[test]
    fn test_invalid_number_returns_error() {
        let source = "create x = 123abc";
        let mut lexer = Lexer::new(source.to_string());

        assert_eq!(lexer.next_token().unwrap(), Token::Create);
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::Equals);

        let result = lexer.next_token();
        
        assert!(result.is_err());
    }
}