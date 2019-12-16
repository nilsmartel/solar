pub fn is_keyword(s: &str) -> bool {
    match s {
        "when" | "is" | "then" | "case" | "if" | "match" | "do" | "loop" | "branch" | "default"
        | "function" | "ret" | "let" => true,
        _ => false,
    }
}

use crate::parse::util::ws_terminated;
use nom::{bytes::complete::tag, IResult};

pub fn tag_loop(i: &str) -> IResult<&str, &str> {
    ws_terminated(tag("loop"))(i)
}

pub fn tag_do(i: &str) -> IResult<&str, &str> {
    ws_terminated(tag("do"))(i)
}

pub fn tag_ret(i: &str) -> IResult<&str, &str> {
    ws_terminated(tag("ret"))(i)
}

pub fn tag_branch(i: &str) -> IResult<&str, &str> {
    ws_terminated(tag("branch"))(i)
}

pub fn tag_default(i: &str) -> IResult<&str, &str> {
    ws_terminated(tag("default"))(i)
}

pub fn tag_let(i: &str) -> IResult<&str, &str> {
    ws_terminated(tag("let"))(i)
}

pub fn tag_is(i: &str) -> IResult<&str, &str> {
    ws_terminated(tag("is"))(i)
}
pub fn tag_then(i: &str) -> IResult<&str, &str> {
    ws_terminated(tag("then"))(i)
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
