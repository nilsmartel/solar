use crate::parse::{
    instructions::{ControlFlow, Expression},
    keyword::tag_do,
    terminals::Identifier,
    Parse,
};

/// do abc = <expr>;
#[derive(Clone, Debug)]
pub struct AssignStatement {
    variable: Identifier,
    kind: AssignmentType,
    assign: Box<Expression>,
}

impl Parse for AssignStatement {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::combinator::map;
        use nom::sequence::preceded;
        use nom::sequence::tuple;

        map(
            tuple((
                preceded(tag_do, Identifier::parse_ws),
                AssignmentType::parse_ws,
                map(Expression::parse_ws, Box::new),
            )),
            |(variable, kind, assign)| AssignStatement {
                variable,
                kind,
                assign,
            },
        )(input)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AssignmentType {
    Assign,
    Add,
    Subtract,
}

impl Parse for AssignmentType {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::{branch::alt, bytes::complete::tag, combinator::map};

        alt((
            map(tag("="), |_| AssignmentType::Assign),
            map(tag("+="), |_| AssignmentType::Add),
            map(tag("-="), |_| AssignmentType::Subtract),
        ))(input)
    }
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
