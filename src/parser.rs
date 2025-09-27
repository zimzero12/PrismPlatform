use crate::ast::{Expression, Program, Statement};
use crate::error::{ParseError, TraceResult};
use crate::token::Token;
use crate::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> TraceResult<Self> {
        let mut parser = Parser {
            lexer,
            current_token: Token::EndOfFile,
        };
        parser.next_token()?;
        Ok(parser)
    }

    fn next_token(&mut self) -> TraceResult<()> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    pub fn parse_program(&mut self) -> TraceResult<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::EndOfFile {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token()?;
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> TraceResult<Statement> {
        match self.current_token {
            Token::Say => self.parse_say_statement(),
            Token::Create => self.parse_create_statement(),
            _ => Err(Box::new(ParseError::UnexpectedToken {
                expected: "a keyword like 'say' or 'create'".to_string(),
                found: self.current_token.clone(),
            })),
        }
    }
    
    fn expect_token(&mut self, expected: Token) -> TraceResult<()> {
        if self.current_token == expected {
            self.next_token()?;
            Ok(())
        } else {
            Err(Box::new(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: self.current_token.clone(),
            }))
        }
    }

    fn parse_say_statement(&mut self) -> TraceResult<Statement> {
        self.next_token()?; // Consume 'say'
        let value = self.parse_expression()?;
        Ok(Statement::SayStatement { value })
    }

    fn parse_create_statement(&mut self) -> TraceResult<Statement> {
        self.next_token()?; // Consume 'create'

        let variable_name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(Box::new(ParseError::UnexpectedToken {
                expected: "a variable name".to_string(),
                found: self.current_token.clone(),
            }));
        };
        self.next_token()?;

        self.expect_token(Token::Equals)?; // Expect and consume '='

        let value = self.parse_expression()?;
        Ok(Statement::CreateStatement { name: variable_name, value })
    }

    fn parse_expression(&mut self) -> TraceResult<Expression> {
        match &self.current_token {
            Token::Identifier(name) => Ok(Expression::Identifier(name.clone())),
            Token::Number(value) => Ok(Expression::NumberLiteral(*value)),
            Token::Text(value) => Ok(Expression::TextLiteral(value.clone())),
            _ => Err(Box::new(ParseError::UnexpectedToken {
                expected: "an expression (variable, number, or text)".to_string(),
                found: self.current_token.clone(),
            })),
        }
    }
}

// in src/parser.rs (at the very bottom)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Statement; // Cleaned up 'use' statement
    use crate::lexer::Lexer;

    // Test 1: A valid create statement
    #[test]
    fn test_create_statement() -> TraceResult<()> {
        let source = "create score = 100";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        assert_eq!(program.statements.len(), 1);
        let statement = &program.statements[0];

        // We can use assert!(matches!(...)) for a cleaner test.
        assert!(matches!(statement, Statement::CreateStatement { name, .. } if name == "score"));
        Ok(())
    }

    // Test 2: A valid say statement
    #[test]
    fn test_say_statement() -> TraceResult<()> {
        let source = "say my_variable";
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;

        assert_eq!(program.statements.len(), 1);
        let statement = &program.statements[0];
        assert!(matches!(statement, Statement::SayStatement { .. }));
        Ok(())
    }

    // Test 3: Ensure a syntax error (missing '=') is caught
    #[test]
    fn test_malformed_create_statement_fails() {
        let source = "create score 100"; // Missing '='
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();

        // We expect parse_program() to return an Err
        let result = parser.parse_program();
        assert!(result.is_err());
    }

    // Test 4: Ensure a statement with an unknown keyword fails
    #[test]
    fn test_unknown_keyword_fails() {
        let source = "delete score"; // 'delete' is not a known keyword
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer).unwrap();

        let result = parser.parse_program();
        assert!(result.is_err());
    }
}