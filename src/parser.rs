use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

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

// in src/parser.rs

pub fn parse_program(&mut self) -> Program {
    let mut program = Program {
        statements: Vec::new(),
    };

    // The loop now correctly continues until the very end of the file.
    while self.current_token != Token::EndOfFile { // <-- THIS LINE IS FIXED
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
            Token::Create => self.parse_create_statement(),
            _ => None,
        }
    }

    fn parse_say_statement(&mut self) -> Option<Statement> {
        self.next_token(); // Move past 'say'
        let value = self.parse_expression()?; // Parse the expression that follows
        Some(Statement::SayStatement { value })
    }

    fn parse_create_statement(&mut self) -> Option<Statement> {
        self.next_token(); // Move past 'create'

        let variable_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return None; // Expected a variable name
        };

        self.next_token();

        if self.current_token != Token::Equals {
            return None; // Expected '='
        }

        self.next_token();

        let value = self.parse_expression()?;
        Some(Statement::CreateStatement { name: variable_name, value })
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        match &self.current_token {
            Token::Identifier(name) => Some(Expression::Identifier(name.clone())),
            Token::Number(value) => Some(Expression::NumberLiteral(*value)),
            Token::Text(value) => Some(Expression::TextLiteral(value.clone())),
            _ => None,
        }
    }
}