use crate::parse::{
    instructions::Statement,
    signature::{FunctionDecl, VariableDecl},
};

#[derive(Clone, Debug)]
pub struct ControlFlow {
    functions: Vec<FunctionDecl>,
    // TODO Vec<vars | statements>
    steps: Vec<DeclarationOrStatement>,
}

#[derive(Clone, Debug)]
enum DeclarationOrStatement {
    Decl(VariableDecl),
    Statement(Statement),
}
