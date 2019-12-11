use crate::parse::{instructions::Expression, signature::Type, terminals::Identifier, Parse};
#[derive(Clone, Debug)]
pub struct VariableDecl {
    name: Identifier,
    kind: Type,
    assign: Option<Box<Expression>>,
}

impl Parse for VariableDecl {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        use nom::{
            bytes::complete::tag,
            character::complete::one_of,
            combinator::{map, opt},
            sequence::{preceded, terminated, tuple},
        };

        use crate::parse::util::char_ws;

        map(
            tuple((
                preceded(
                    terminated(tag("let"), one_of(" \n\t\r")),
                    Identifier::parse_ws,
                ),
                preceded(char_ws(':'), Type::parse_ws),
                opt(preceded(
                    char_ws('='),
                    map(Expression::parse_ws, |e| Box::new(e)),
                )),
            )),
            |(name, kind, assign)| VariableDecl { name, kind, assign },
        )(input)
    }
}
