type ParseFunc<I, O> = fn(input: I) -> nom::IResult<I, O>;
pub fn test_parser<'a, I, O>(test: ParseFunc<I, O>, cases: Vec<(&'a str, O)>)
where
    I: std::fmt::Debug,
    I: std::convert::From<&'a str>,
    O: std::fmt::Debug,
    O: std::cmp::PartialEq,
{
    for (index, (input, expected)) in cases.iter().enumerate() {
        let actual = (test)(input.clone().into());
        let (_, actual) = actual.expect(&format!(
            "[{}]: failed to parse expression \"{}\"",
            index, input
        ));
        assert_eq!(actual, *expected, "\ninput [{}]: `{}`", index, input);
    }
}
