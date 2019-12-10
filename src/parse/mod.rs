mod keyword;
mod terminals;
mod traits;
mod util;
use keyword::is_keyword;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;
use terminals::Identifier;
use traits::Parse;
use util::char_ws;

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

#[derive(Clone, Debug, Eq, PartialEq)]
struct FunctionSignature {
    name: Identifier,
    params: Vec<Parameter>,
}

#[derive(Clone, Debug)]
struct Expression {}

/// do abc = <expr>;
#[derive(Clone, Debug)]
struct AssignStatement {
    variable: Identifier,
    assign: Box<Expression>,
}

#[derive(Clone, Debug)]
struct BranchStatement {
    condition: Box<Expression>,
    body: Box<FunctionBody>,
    default: Option<Box<FunctionBody>>,
}

/// do abc = <expr>;
/// do <expr>;
/// branch <expression> { <function_body> }     [ default { <function_body> } ]?
/// return  [ <expression> ]?
/// loop { <function_body> }
#[derive(Clone, Debug)]
enum Statement {
    Assign(AssignStatement),
    Branch(BranchStatement),
    Return(Box<Expression>),
    Loop(Box<FunctionBody>),
}

#[derive(Clone, Debug)]
struct VariableDecl {}

#[derive(Clone, Debug)]
struct FunctionBody {
    variables: Vec<VariableDecl>,
    functions: Vec<FunctionDecl>,
    statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
struct FunctionDecl {
    signature: FunctionSignature,
    body: FunctionBody,
}
