use crate::parse::{
    instructions::Statement,
    signature::{FunctionDecl, VariableDecl},
    Parse,
};
use nom::branch::alt;
use nom::combinator::map;

#[derive(Clone, Debug)]
pub struct ControlFlow {
    functions: Vec<FunctionDecl>,
    // TODO Vec<vars | statements>
    steps: Vec<DeclarationOrStatement>,
}

impl Parse for ControlFlow {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
enum DeclarationOrStatement {
    Decl(VariableDecl),
    Statement(Statement),
}

impl Parse for DeclarationOrStatement {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        alt((
            map(VariableDecl::parse, DeclarationOrStatement::Decl),
            map(Statement::parse, DeclarationOrStatement::Statement),
        ))(input)
    }
}
