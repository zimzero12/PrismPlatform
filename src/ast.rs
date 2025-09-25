// in src/ast.rs

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// Represents a value or computation
#[derive(Debug)]
pub enum Expression {
    NumberLiteral(f64),
    // We'll add TextLiteral, Identifier, etc. here later
}

// Represents an action or command
#[derive(Debug)]
pub enum Statement {
    SayStatement { value: String },
    CreateStatement { // <-- ADDED
        name: String,
        value: Expression,
    },
}