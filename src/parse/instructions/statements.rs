use crate::parse::{
    instructions::{ControlFlow, Expression},
    terminals::Identifier,
};

/// do abc = <expr>;
#[derive(Clone, Debug)]
pub struct AssignStatement {
    variable: Identifier,
    assign: Box<Expression>,
}

#[derive(Copy, Clone)]
pub enum AssignmentType {
    Assign,
}

#[derive(Clone, Debug)]
pub struct BranchStatement {
    condition: Box<Expression>,
    body: Box<ControlFlow>,
    default: Option<Box<ControlFlow>>,
}

/// do abc = <expr>;
/// do <expr>;
/// branch <expression> { <function_body> }     [ default { <function_body> } ]?
/// return  [ <expression> ]?
/// loop { <function_body> }
#[derive(Clone, Debug)]
pub enum Statement {
    Assign(AssignStatement),
    Branch(BranchStatement),
    Return(Box<Expression>),
    Loop(Box<ControlFlow>),
}
