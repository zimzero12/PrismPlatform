use crate::ast::{Program, Statement};
use crate::lexer::Lexer;
use crate::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EndOfFile, // Placeholder
        };
        // Load the first token to get started.
        parser.next_token();
        parser
    }

    // A helper function to advance to the next token from the lexer.
    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    // The main function of the parser. It builds the entire AST.
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        // Loop through all the tokens until we reach the end.
        while self.current_token != Token::EndOfFile {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        program
    }

    // This function figures out what kind of statement we're looking at.
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Say => self.parse_say_statement(),
            _ => None, // If we don't recognize it, we ignore it for now.
        }
    }

    // This handles the grammar for a "say" statement.
    // It expects the current token to be 'Say' and the next to be 'Text'.
    fn parse_say_statement(&mut self) -> Option<Statement> {
        self.next_token(); // Move from 'say' to the next token

        if let Token::Text(value) = &self.current_token {
            Some(Statement::SayStatement {
                value: value.clone(),
            })
        } else {
            // If the token after 'say' is not Text, it's a grammar error.
            None
        }
    }
}