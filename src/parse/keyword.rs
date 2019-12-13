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
    tag_terminated_word("let", i)
}

fn tag_terminated_word<'a>(word: &'a str, input: &'a str) -> IResult<&'a str, &'a str> {
    ws_terminated(tag(word))(input)
}
