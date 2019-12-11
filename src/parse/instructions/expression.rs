use crate::parse::Parse;
#[derive(Clone, Debug)]
pub struct Expression {}

impl Parse for Expression {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        unimplemented!()
    }
}
