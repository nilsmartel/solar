/// is expression
///
/// an is expression returns an Fn<T, R>(t) -> R
use crate::parse::{instructions::Expression, keyword, util::ignore_ws, Parse};
use nom::{
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

pub struct IsExpression {
    cases: Vec<Case>,
}

impl Parse for IsExpression {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(nom::multi::many1(Case::parse_ws), |cases| IsExpression {
            cases,
        })(input)
    }
}

struct Case {
    matcher: Box<Expression>,
    then: Box<Expression>,
}

impl Parse for Case {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                preceded(keyword::tag_is, map(Expression::parse_ws, Box::new)),
                preceded(
                    ignore_ws(keyword::tag_then),
                    map(Expression::parse_ws, Box::new),
                ),
            )),
            |(matcher, then)| Case { matcher, then },
        )(input)
    }
}
