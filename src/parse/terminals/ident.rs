use crate::parse::keyword::is_keyword;
use crate::parse::traits::Parse;
use nom::character::complete::{alpha1, alphanumeric0};
use nom::combinator::map;
use nom::sequence::pair;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Hash)]
pub struct Identifier(String);
impl Parse for Identifier {
    /// Recognize Identifiers,
    /// Escapes keywords
    fn parse(input: &str) -> IResult<&str, Identifier> {
        let (rest, identifier) =
            // TODO allow _underscores_ to be parsed
            map(pair(alpha1, alphanumeric0), |(a, b)| format!("{}{}", a, b))(input)?;

        if is_keyword(&identifier) {
            return Err(nom::Err::Error((input, nom::error::ErrorKind::Tag)));
        }

        Ok((rest, Identifier(identifier)))
    }
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Identifier(s.into())
    }
}

impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Identifier(s)
    }
}
