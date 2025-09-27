// in src/error.rs

use crate::token::Token;

// This is the main error type for our entire language.
// We use 'Box<dyn ...>' to allow for different kinds of specific errors.
pub type TraceResult<T> = Result<T, Box<dyn std::error::Error>>;

// A more specific error enum for parsing failures.
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token },
    InvalidNumber(String),
    // We can add more specific parsing errors here later.
}

// This is required to make our ParseError a valid error type.
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "Parse Error: Expected {} but found {:?}", expected, found)
            }
            ParseError::InvalidNumber(num_str) => {
                write!(f, "Parse Error: Could not parse '{}' as a valid number.", num_str)
            }
        }
    }
}
impl std::error::Error for ParseError {}


// A more specific error enum for evaluation failures.
#[derive(Debug)]
pub enum EvalError {
    VariableNotFound(String),
    // We will add TypeMismatch, DivisionByZero, etc. here later.
}

// This is required to make our EvalError a valid error type.
impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EvalError::VariableNotFound(name) => {
                write!(f, "Runtime Error: Variable '{}' not found.", name)
            }
        }
    }
}
impl std::error::Error for EvalError {}