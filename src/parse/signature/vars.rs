use crate::parse::{instructions::Expression, signature::Type, terminals::Identifier};
#[derive(Clone, Debug)]
pub struct VariableDecl {
    name: Identifier,
    kind: Type,
    assign: Option<Box<Expression>>,
}
