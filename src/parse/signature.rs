use crate::parse::{terminals::Identifier, traits::Parse, util::char_ws};
use nom::{combinator::map, sequence::separated_pair, IResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Type {
    name: Identifier,
}

impl Parse for Type {
    fn parse(input: &str) -> IResult<&str, Type> {
        nom::combinator::map(Identifier::parse, |name| Type { name })(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub name: Identifier,
    pub kind: Type,
}

impl Parse for Parameter {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        map(
            separated_pair(Identifier::parse, char_ws(':'), Type::parse_ws),
            |(name, kind)| Parameter { name, kind },
        )(input)
    }
}

#[cfg(test)]
mod test_paramter {
    use super::*;
    #[test]
    fn basic() {
        let input = "myvar: string";
        let (rest, result) = Parameter::parse(input).unwrap();

        assert_eq!(
            result,
            Parameter {
                name: "myvar".into(),
                kind: Type {
                    name: "string".into()
                }
            }
        );

        assert_eq!(rest, "");
    }
}
