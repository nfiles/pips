//! Parser Module

use nom;
use nom::digit;
use nom::types::CompleteStr;

use std::str::FromStr;

use crate::expression::Expression;
use Expression::*;

named!(
    parse_signs<CompleteStr, CompleteStr>,
    ws!(take_while!(call!(|c| c == '+' || c == '-')))
);

named!(
    parse_eval_signs<CompleteStr, char>,
    map!(parse_signs, |input: CompleteStr| {
        let neg_count = input.chars().filter(|&x| x == '-').count();
        match neg_count % 2 {
            1 => '-',
            _ => '+',
        }
    })
);

named!(
    parse_unsigned_number<CompleteStr, u32>,
    map_res!(
        recognize!(digit),
        |CompleteStr(string)| u32::from_str(string)
    )
);

named!(
    parse_signed_number<CompleteStr, i32>,
    map!(
        pair!(
            opt!(parse_eval_signs),
            parse_unsigned_number
        ),
        |(sign, value): (Option<char>, u32)| {
            match sign {
                Some('-') => -1 * value as i32,
                _ => 1 * value as i32
            }
        }
    )
);

named!(
    parse_constant<CompleteStr, Expression>,
    map!(parse_signed_number, |num: i32| Constant(num))
);

named!(
    parse_die<CompleteStr, Expression>,
    do_parse!(
        ws!(tag!("d")) >>
        num: parse_unsigned_number >>
        (Die(num as i32))
    )
);

named!(
    parse_parens<CompleteStr, Expression>,
    ws!(delimited!(tag!("("), parse_expression, tag!(")")))
);

named!(
    parse_expression<CompleteStr, Expression>,
    alt_complete!(
        parse_parens |
        parse_die |
        parse_constant
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_success() {
        let cases = vec![
            ("-999999999", -999999999),
            ("-1234", -1234),
            ("---10", -10),
            ("-10", -10),
            ("-1", -1),
            ("0", 0),
            ("1", 1),
            ("+++--1", 1),
            ("10", 10),
            ("1234", 1234),
            ("--1234", 1234),
            ("999999999", 999999999),
        ];

        for (input, expected) in cases {
            let actual = parse_signed_number(input.into());
            let (_, actual) = actual.unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_constant() {
        let cases = vec![
            ("4", Constant(4)),
            ("  5  ", Constant(5)),
            ("\n\t --10  ", Constant(10)),
            ("\n\t ---10  ", Constant(-10)),
        ];

        for (input, expected) in cases {
            let actual = parse_constant(input.into());
            let (_, actual) = actual.unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_die() {
        let cases = vec![
            ("d4", Die(4)),
            ("  d6", Die(6)),
            (" d10", Die(10)),
            ("\nd12", Die(12)),
            ("\td20  ", Die(20)),
        ];

        for (input, expected) in cases {
            let actual = parse_die(input.into());
            let (_, actual) = actual.unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_parens() {
        let cases = vec![
            ("(d4)", Die(4)),
            ("\t(\n--12)", Constant(12)),
            ("  (  ---+20)  ", Constant(-20)),
            (" ( d20 ) ", Die(20)),
        ];

        for (input, expected) in cases {
            let actual = parse_parens(input.into());
            let (_, actual) = actual.unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_expression() {
        let cases = vec![
            ("d4", Die(4)),
            ("4", Constant(4)),
            ("\t\n ---12", Constant(-12)),
            ("\t\n d20", Die(20)),
            (" ( d20 ) ", Die(20)),
        ];

        for (input, expected) in cases {
            let actual = parse_expression(input.into());
            let (_, actual) = actual.unwrap();
            assert_eq!(expected, actual);
        }
    }
}
