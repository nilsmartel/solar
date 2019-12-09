pub fn is_keyword(s: &str) -> bool {
    match s {
        "do" | "loop" | "branch" | "default" | "function" | "ret" | "let" => true,
        _ => false,
    }
}
