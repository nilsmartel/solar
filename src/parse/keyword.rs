pub fn is_keyword(s: &str) -> bool {
    match s {
        "when" | "case" | "if" | "match" | "do" | "loop" | "branch" | "default" | "function"
        | "ret" | "let" => true,
        _ => false,
    }
}

use crate::parse::util::ws_terminated;
use nom::{bytes::complete::tag, IResult};

pub fn tag_do(i: &str) -> IResult<&str, &str> {
    tag_terminated_word("do", i)
}
pub fn tag_branch(i: &str) -> IResult<&str, &str> {
    tag_terminated_word("branch", i)
}
pub fn tag_default(i: &str) -> IResult<&str, &str> {
    tag_terminated_word("default", i)
}
pub fn tag_let(i: &str) -> IResult<&str, &str> {
    ws_terminated(tag("let"))(i)
}

fn tag_terminated_word<'a>(word: &'a str, input: &'a str) -> IResult<&'a str, &'a str> {
    ws_terminated(tag(word))(input)
}

#[cfg(test)]
mod keyword_tests {
    use super::*;

    #[test]
    fn tag_keywords() {
        assert_eq!(Ok(("something", "let")), tag_let("let something"));
        assert_eq!(Ok(("", "let")), tag_let("let "));
        assert_eq!(Ok(("", "do")), tag_do("do "));
        assert_eq!(Ok(("", "branch")), tag_branch("branch "));
        assert_eq!(Ok(("", "default")), tag_default("default "));
    }
}
