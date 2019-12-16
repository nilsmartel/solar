use crate::parse::{
    instructions::{ControlFlow, Expression},
    keyword,
    keyword::tag_do,
    terminals::Identifier,
    util::char_ws,
    util::tag_ws,
    Parse,
};

use nom::combinator::opt;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::{branch::alt, bytes::complete::tag, combinator::map};

/// do abc = <expr>;
#[derive(Clone, Debug)]
pub struct AssignStatement {
    variable: Identifier,
    kind: AssignmentType,
    assign: Box<Expression>,
}

impl Parse for AssignStatement {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
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

impl Parse for BranchStatement {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        map(
            tuple((
                map(
                    preceded(keyword::tag_branch, Expression::parse_ws),
                    Box::new,
                ),
                map(
                    delimited(char_ws('{'), ControlFlow::parse_ws, char_ws('}')),
                    Box::new,
                ),
                opt(preceded(
                    keyword::tag_default,
                    map(
                        delimited(char_ws('{'), ControlFlow::parse_ws, char_ws('}')),
                        Box::new,
                    ),
                )),
            )),
            |(condition, body, default)| BranchStatement {
                condition,
                body,
                default,
            },
        )(input)
    }
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

impl Parse for Statement {
    fn parse(input: &str) -> IResult<&str, Statement> {
        nom::branch::alt((
            map(AssignStatement::parse, Statement::Assign),
            map(BranchStatement::parse, Statement::Branch),
            map(
                preceded(keyword::tag_ret, map(Expression::parse_ws, Box::new)),
                Statement::Return,
            ),
            map(
                preceded(
                    keyword::tag_loop,
                    delimited(
                        char_ws('{'),
                        map(ControlFlow::parse_ws, Box::new),
                        char_ws('}'),
                    ),
                ),
                Statement::Loop,
            ),
        ))(input)
    }
}
