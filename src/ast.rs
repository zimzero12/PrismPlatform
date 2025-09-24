// The core building block of our AST. For now, a program is just a list of statements.
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// This enum represents all the different kinds of statements our language can have.
// Right now, it only has one: a "say" statement.
#[derive(Debug)]
pub enum Statement {
    SayStatement { value: String }, // e.g., say "this is the value"
    // We will add LetStatement, IfStatement, etc. here later.
}