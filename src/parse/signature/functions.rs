use crate::parse::{instructions::ControlFlow, signature::Parameter, terminals::Identifier};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionSignature {
    name: Identifier,
    params: Vec<Parameter>,
}

#[derive(Clone, Debug)]
pub struct FunctionDecl {
    signature: FunctionSignature,
    body: ControlFlow,
}
