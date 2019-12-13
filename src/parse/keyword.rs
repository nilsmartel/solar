pub fn is_keyword(s: &str) -> bool {
    match s {
        "do" | "loop" | "branch" | "default" | "function" | "ret" | "let" => true,
        _ => false,
    }
}

use crate::parse::util::*;

pub fn tag_do(i: &str) -> nom::IResult<&str, &str> {
    not_followed(
        nom::bytes::complete::tag("do"),
        nom::character::complete::one_of(" \n\r\t"),
    )(i)
}
