use crate::parse::{
    instructions::ControlFlow,
    signature::{Parameter, Type},
    terminals::Identifier,
    traits::Parse,
    util::{char_ws, tag_ws},
};
use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map, map_res, opt},
    multi::separated_list,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionSignature {
    name: Identifier,
    params: Vec<Parameter>,
    return_type: Option<Type>,
}

impl Parse for FunctionSignature {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                preceded(
                    terminated(tag("function"), one_of(" \n\r\t")),
                    Identifier::parse_ws,
                ),
                delimited(
                    char_ws('('),
                    separated_list(char_ws(','), Parameter::parse_ws),
                    char_ws(')'),
                ),
                opt(preceded(tag_ws("->"), Type::parse_ws)),
            )),
            |(name, params, return_type)| FunctionSignature {
                name,
                params,
                return_type,
            },
        )(input)
    }
}

#[cfg(test)]
mod test_function_signature {
    use super::FunctionSignature;
    use crate::parse::{
        signature::{Parameter, Type},
        Parse,
    };
    #[test]
    fn basic() {
        let input = "function mid(first: float, second: float) -> float";
        let desired = FunctionSignature {
            name: "mid".into(),
            params: vec![
                Parameter {
                    name: "first".into(),
                    kind: Type::parse("float").unwrap().1,
                },
                Parameter {
                    name: "second".into(),
                    kind: Type::parse("float").unwrap().1,
                },
            ],
            return_type: Some(Type::parse("float").unwrap().1),
        };

        let (rest, sig) = FunctionSignature::parse(input).unwrap();

        assert_eq!(sig, desired);
        assert_eq!(rest, "");
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDecl {
    signature: FunctionSignature,
    body: ControlFlow,
}
