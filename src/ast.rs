#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// Represents a value or computation. Added Clone to make it easier to pass around.
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),   // For variable names like 'score'
    NumberLiteral(f64),
    TextLiteral(String),  // For quoted text like "Hello"
}

// Represents an action or command.
#[derive(Debug)]
pub enum Statement {
    // MODIFIED: 'value' is now an Expression, not a raw String.
    SayStatement { value: Expression },
    CreateStatement {
        name: String,
        value: Expression,
    },
}