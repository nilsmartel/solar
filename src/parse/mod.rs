type Identifier = String;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Parameter {
    name: Identifier,
    kind: Identifier,
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
