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
    parse_die_single<CompleteStr, Expression>,
    do_parse!(
             ws!(tag!("d"))        >>
        num: parse_unsigned_number >>
        (Die(num))
    )
);

// named!(
//     parse_die_coefficient<CompleteStr, Expression>,
//     map!(
//         do_parse!(
//             coefficient: ws!(parse_unsigned_number) >>
//                          tag!("d")                  >>
//             num:         parse_unsigned_number      >>
//             (coefficient, num)
//         ),
//         |(coefficient, num)| {
//             if coefficient == 0 {
//                 return Constant(0)
//             }

//             (0..coefficient).fold(Die(num), |acc, _| {
//                 Sum(Box::new(acc), Box::new(Die(num)))
//             })
//         }
//     )
// );

named!(
    parse_die<CompleteStr, Expression>,
    alt_complete!(parse_die_single)
    // alt_complete!(parse_die_single | parse_die_coefficient)
);

named!(
    parse_parens<CompleteStr, Expression>,
    delimited!(
        ws!(tag!("(")),
        // alt_complete!(parse_expression | parse_functions),
        parse_expression,
        ws!(tag!(")"))
    )
);

named!(
    parse_unary_function<CompleteStr, Expression>,
    map!(
        ws!(
            pair!(
                alt_complete!(
                    tag!("dis") |
                    tag!("adv")
                ),
                parse_parens
            )
        ),
        |(CompleteStr(func), expr): (CompleteStr, Expression)| {
            match func {
                "dis" => Disadvantage(Box::new(expr)),
                "adv" => Advantage(Box::new(expr)),
                _ => panic!("unknown unary function")
            }
        }
    )
);

named!(
    parse_sum<CompleteStr, Expression>,
    do_parse!(
        init: parse_multiply >>
        res:  fold_many0!(
            ws!(
                pair!(
                    alt!(tag!("+") | tag!("-")),
                    parse_multiply
                )
            ),
            init,
            |acc, (CompleteStr(op), expr): (CompleteStr, Expression)| {
                // TODO: balance
                match op {
                    "+" => Sum(Box::new(acc), Box::new(expr)),
                    "-" => Diff(Box::new(acc), Box::new(expr)),
                    _ => panic!("unknown operator"),
                }
            }
        ) >>
        (res)
    )
);

named!(
    parse_multiply<CompleteStr, Expression>,
    do_parse!(
        init: parse_root >>
        res:  fold_many0!(
            ws!(
                pair!(
                    alt!(tag!("*") | tag!("/")),
                    parse_root
                )
            ),
            init,
            |acc, (CompleteStr(op), expr): (CompleteStr, Expression)| {
                // TODO: balance
                match op {
                    "*" => Multiply(Box::new(acc), Box::new(expr)),
                    "-" => Divide(Box::new(acc), Box::new(expr)),
                    _ => panic!("unknown operator"),
                }
            }
        ) >>
        (res)
    )
);

named!(
    parse_expression<CompleteStr, Expression>,
    alt_complete!(
        // order of operations
        // parse_parens |
        // functions
        // parse_unary_function |
        // multiple/divide
        // add/subtract
        // parse_sum |
        parse_multiply |
        parse_sum
        // parse_root
        // parse_die |
        // parse_constant
    )
);

named!(
    parse_root<CompleteStr, Expression>,
    alt_complete!(
        parse_parens |
        parse_die |
        parse_constant
    )
);

named!(
    parse_functions<CompleteStr, Expression>,
    alt_complete!(
        parse_unary_function
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
            (" ( ( d20 ) ) ", Die(20)),
            (" ( ( ( d20 ) ) ) ", Die(20)),
        ];

        for (input, expected) in cases {
            let actual = parse_parens(input.into());
            let (_, actual) = actual.unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_unary_function() {
        let cases = vec![
            ("dis(d4)", Disadvantage(Box::new(Die(4)))),
            (" dis ( d4 ) ", Disadvantage(Box::new(Die(4)))),
            ("adv(d4)", Advantage(Box::new(Die(4)))),
            (" adv( d4 ) ", Advantage(Box::new(Die(4)))),
            (
                " adv ( d20 + d4 ) ",
                Advantage(Box::new(Sum(Box::new(Die(20)), Box::new(Die(4))))),
            ),
        ];

        for (input, expected) in cases {
            let actual = parse_unary_function(input.into());
            let (_, actual) = actual.unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_parse_sum() {
        let cases = vec![
            ("d4+d4", Sum(Box::new(Die(4)), Box::new(Die(4)))),
            (" d20\t\n + d4 ", Sum(Box::new(Die(20)), Box::new(Die(4)))),
            ("d20 + 4", Sum(Box::new(Die(20)), Box::new(Constant(4)))),
            (
                "d20 + d10 + d4 + 4",
                Sum(
                    Box::new(Sum(
                        Box::new(Sum(Box::new(Die(20)), Box::new(Die(10)))),
                        Box::new(Die(4)),
                    )),
                    Box::new(Constant(4)),
                ),
            ),
            ("d4-1", Diff(Box::new(Die(4)), Box::new(Constant(1)))),
            (" d12 - d4 ", Diff(Box::new(Die(12)), Box::new(Die(4)))),
            (
                "d20 + d10 - d4 + 4",
                Sum(
                    Box::new(Diff(
                        Box::new(Sum(Box::new(Die(20)), Box::new(Die(10)))),
                        Box::new(Die(4)),
                    )),
                    Box::new(Constant(4)),
                ),
            ),
            (" d20 ", Die(20)),
        ];

        for (input, expected) in cases {
            let actual = parse_sum(input.into());
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
            (" d20\t\n + d4 ", Sum(Box::new(Die(20)), Box::new(Die(4)))),
            (
                "2*d10+d4",
                Sum(
                    Box::new(Multiply(Box::new(Constant(2)), Box::new(Die(10)))),
                    Box::new(Die(4)),
                ),
            ),
            (
                "2*d10+(adv(d20)-d4)*d4",
                Sum(
                    Box::new(Multiply(Box::new(Constant(2)), Box::new(Die(10)))),
                    Box::new(Multiply(
                        Box::new(Diff(
                            Box::new(Advantage(Box::new(Die(20)))),
                            Box::new(Die(4)),
                        )),
                        Box::new(Die(4)),
                    )),
                ),
            ),
        ];

        for (input, expected) in cases {
            let actual = parse_expression(input.into());
            let (_, actual) = actual.unwrap();
            assert_eq!(actual, expected);
        }
    }
}
