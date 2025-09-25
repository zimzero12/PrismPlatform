// in src/parser.rs

use crate::ast::{Expression, Program, Statement};
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
            current_token: Token::EndOfFile,
        };
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::EndOfFile {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Say => self.parse_say_statement(),
            Token::Create => self.parse_create_statement(), // <-- ADDED
            _ => None,
        }
    }

    fn parse_say_statement(&mut self) -> Option<Statement> {
        self.next_token();
        if let Token::Text(value) = &self.current_token {
            Some(Statement::SayStatement {
                value: value.clone(),
            })
        } else {
            None
        }
    }

    // NEW FUNCTION to parse "create" statements
    fn parse_create_statement(&mut self) -> Option<Statement> {
        self.next_token(); // Move past 'create'

        let variable_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return None; // Grammar error: expected a variable name
        };

        self.next_token(); // Move past the name

        if self.current_token != Token::Equals {
            return None; // Grammar error: expected '='
        }

        self.next_token(); // Move past '='

        let value = self.parse_expression()?; // Parse the value on the right side

        Some(Statement::CreateStatement { name: variable_name, value })
    }

    // NEW FUNCTION to parse any kind of value/expression
    fn parse_expression(&mut self) -> Option<Expression> {
        match &self.current_token {
            Token::Number(_) => self.parse_number_literal(),
            _ => None, // We only support numbers for now
        }
    }

    // NEW FUNCTION specifically for parsing numbers
    fn parse_number_literal(&mut self) -> Option<Expression> {
        if let Token::Number(value) = self.current_token {
            Some(Expression::NumberLiteral(value))
        } else {
            None
        }
    }
}