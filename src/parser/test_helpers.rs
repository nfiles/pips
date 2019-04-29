#[cfg(test)]
type ParseFunc<I, O> = fn(input: I) -> nom::IResult<I, O>;

#[cfg(test)]
pub fn test_parser<'a, I, O>(test: ParseFunc<I, O>, cases: Vec<(&'a str, O)>)
where
    I: std::fmt::Debug,
    I: std::convert::From<&'a str>,
    I: std::fmt::Display,
    O: std::fmt::Debug,
    O: std::cmp::PartialEq,
{
    for (index, (input, expected)) in cases.iter().enumerate() {
        let actual = (test)(input.clone().into());
        let (rest, actual) = actual.expect(&format!(
            "[{}]: failed to parse expression \"{}\"",
            index, input
        ));
        assert_eq!(
            actual,
            *expected,
            "\ninput [{index}]: `{input}`\nrest  [{index}]: `{rest}`",
            index = index,
            input = input,
            rest = rest
        );
    }
}
