mod keyword;
mod util;

use keyword::is_keyword;
use nom::character::complete::alpha1;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Hash)]
pub struct Identifier(String);
impl Identifier {
    /// Recognize Identifiers,
    /// Escapes keywords
    pub fn parse(input: &str) -> IResult<&str, Identifier> {
        let (rest, identifier) = alpha1(input)?;

        if is_keyword(identifier) {
            return Err(nom::Err::Error((input, nom::error::ErrorKind::Tag)));
        }

        Ok((rest, Identifier(identifier.to_string())))
    }

    /// Recognize Identifiers,
    /// Escape keywords,
    /// Ignore Whitespace
    pub fn parse_ws(input: &str) -> IResult<&str, Identifier> {
        util::ignore_ws(Identifier::parse)(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Type {
    name: Identifier,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Parameter {
    name: Identifier,
    kind: Type,
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
