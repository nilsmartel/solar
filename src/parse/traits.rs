pub trait Parse
where
    Self: std::marker::Sized,
{
    fn parse(input: &str) -> nom::IResult<&str, Self>;

    fn parse_ws(input: &str) -> nom::IResult<&str, Self> {
        crate::parse::util::ignore_ws(Self::parse)(input)
    }
}
