//! Parser Module

use nom;
use nom::types::CompleteStr;

use crate::expression::Comparison;
use crate::expression::Expression;
use Comparison::*;
use Expression::*;

use super::base_terms::{parse_constant, parse_die};

named!(
    parse_base_term<CompleteStr, Expression>,
    ws!(
        alt_complete!(
            parse_functions |
            parse_parens |
            parse_die |
            parse_constant
        )
    )
);

named!(
    parse_parens<CompleteStr, Expression>,
    delimited!(
        ws!(tag!("(")),
        parse_expression,
        ws!(tag!(")"))
    )
);

named!(
    parse_unary_function<CompleteStr, Expression>,
    map!(
        pair!(
            ws!(
                alt_complete!(
                    tag!("dis") |
                    tag!("adv")
                )
            ),
            parse_parens
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
    parse_functions<CompleteStr, Expression>,
    alt_complete!(
        parse_unary_function
    )
);

named!(
    parse_multiply<CompleteStr, Expression>,
    do_parse!(
        init: parse_base_term >>
        res:  fold_many0!(
            pair!(
                ws!(alt!(tag!("*") | tag!("/"))),
                parse_base_term
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
    parse_sum<CompleteStr, Expression>,
    do_parse!(
        init: parse_multiply >>
        res:  fold_many0!(
            pair!(
                ws!(alt!(tag!("+") | tag!("-"))),
                parse_multiply
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
    parse_comparison<CompleteStr, Expression>,
    map_res!(
        do_parse!(
            left: ws!(parse_sum) >>
            operator: ws!(
                alt_complete!(
                    tag!(">=") |
                    tag!(">") |
                    tag!("<=") |
                    tag!("<") |
                    tag!("=")
                )
            ) >>
            right: ws!(parse_sum) >>
            (left, right, operator)
        ),
        |(left, right, CompleteStr(operator)): (Expression, Expression, CompleteStr)| -> Result<Expression, &str> {
            let comparison = match operator {
                ">=" => Ok(GreaterThanOrEqualTo),
                ">" => Ok(GreaterThan),
                "<=" => Ok(LessThanOrEqualTo),
                "<" => Ok(LessThan),
                "=" => Ok(EqualTo),
                _ => Err("unknown comparison operator"),
            }?;

            Ok(Compare(Box::new(left), Box::new(right), comparison))
        }
    )
);

/// recursive parser that starts the order of operations
named!(
    parse_expression<CompleteStr, Expression>,
    alt_complete!(
        parse_comparison |
        parse_sum
    )
);

/// root parser that consumes entire input including trailing whitespace
named!(
    parse_full_expression<CompleteStr, Expression>,
    do_parse!(
        expr:   parse_expression >>
                ws!(eof!())      >>
        (expr)
    )
);

/// parse an input string into an `Expression` and report errors
pub fn parse(input: &str) -> Result<Expression, nom::Err<CompleteStr>> {
    let result = parse_full_expression(input.into());
    match result {
        Ok((_, expr)) => Ok(expr),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::test_helpers::test_parser;

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

        test_parser(parse_parens, cases);
    }

    #[test]
    fn test_parse_unary_function() {
        let cases = vec![
            (
                "dis(2d4)",
                Disadvantage(Box::new(Sum(Box::new(Die(4)), Box::new(Die(4))))),
            ),
            (" dis ( d4 ) ", Disadvantage(Box::new(Die(4)))),
            ("adv(d4)", Advantage(Box::new(Die(4)))),
            (" adv( d4 ) ", Advantage(Box::new(Die(4)))),
            (
                " adv ( d20 + d4 ) ",
                Advantage(Box::new(Sum(Box::new(Die(20)), Box::new(Die(4))))),
            ),
            (
                " adv ( d20 * d10 + d4 * 5 ) ",
                Advantage(Box::new(Sum(
                    Box::new(Multiply(Box::new(Die(20)), Box::new(Die(10)))),
                    Box::new(Multiply(Box::new(Die(4)), Box::new(Constant(5)))),
                ))),
            ),
        ];

        test_parser(parse_unary_function, cases);
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
                "d20 + d10 - d4 + -4 ",
                Sum(
                    Box::new(Diff(
                        Box::new(Sum(Box::new(Die(20)), Box::new(Die(10)))),
                        Box::new(Die(4)),
                    )),
                    Box::new(Constant(-4)),
                ),
            ),
            (" d20 ", Die(20)),
        ];

        test_parser(parse_sum, cases);
    }

    #[test]
    fn test_parse_compare() {
        let cases = vec![
            (
                "d4 > d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), GreaterThan),
            ),
            (
                "d4 >= d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), GreaterThanOrEqualTo),
            ),
            (
                "d4 < d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), LessThan),
            ),
            (
                "d4 <= d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), LessThanOrEqualTo),
            ),
            (
                "d4 = d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), EqualTo),
            ),
        ];

        test_parser(parse_comparison, cases);
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
                " 2 * d10 + d4 ",
                Sum(
                    Box::new(Multiply(Box::new(Constant(2)), Box::new(Die(10)))),
                    Box::new(Die(4)),
                ),
            ),
            (
                " 2 + d10 * d4 ",
                Sum(
                    Box::new(Constant(2)),
                    Box::new(Multiply(Box::new(Die(10)), Box::new(Die(4)))),
                ),
            ),
            (
                "(adv(d20)) + 2",
                Sum(
                    Box::new(Advantage(Box::new(Die(20)))),
                    Box::new(Constant(2)),
                ),
            ),
            (
                "adv(d20) + 2",
                Sum(
                    Box::new(Advantage(Box::new(Die(20)))),
                    Box::new(Constant(2)),
                ),
            ),
            (
                "2+adv(d20)",
                Sum(
                    Box::new(Constant(2)),
                    Box::new(Advantage(Box::new(Die(20)))),
                ),
            ),
            (
                "2*d10+adv(d20)",
                Sum(
                    Box::new(Multiply(Box::new(Constant(2)), Box::new(Die(10)))),
                    Box::new(Advantage(Box::new(Die(20)))),
                ),
            ),
            (
                "2*d10+adv(d20)-d4",
                Diff(
                    Box::new(Sum(
                        Box::new(Multiply(Box::new(Constant(2)), Box::new(Die(10)))),
                        Box::new(Advantage(Box::new(Die(20)))),
                    )),
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
            (
                " adv ( d20 * d10 + d4 ) + 5 ",
                Sum(
                    Box::new(Advantage(Box::new(Sum(
                        Box::new(Multiply(Box::new(Die(20)), Box::new(Die(10)))),
                        Box::new(Die(4)),
                    )))),
                    Box::new(Constant(5)),
                ),
            ),
            (
                " adv ( d20 * d10 + d4 ) + adv ( d20 * d10 + d4 ) ",
                Sum(
                    Box::new(Advantage(Box::new(Sum(
                        Box::new(Multiply(Box::new(Die(20)), Box::new(Die(10)))),
                        Box::new(Die(4)),
                    )))),
                    Box::new(Advantage(Box::new(Sum(
                        Box::new(Multiply(Box::new(Die(20)), Box::new(Die(10)))),
                        Box::new(Die(4)),
                    )))),
                ),
            ),
            (
                "d4 > d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), GreaterThan),
            ),
            (
                "d4 >= d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), GreaterThanOrEqualTo),
            ),
            (
                "d4 < d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), LessThan),
            ),
            (
                "d4 <= d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), LessThanOrEqualTo),
            ),
            (
                "d4 = d4",
                Compare(Box::new(Die(4)), Box::new(Die(4)), EqualTo),
            ),
        ];

        test_parser(parse_expression, cases);
    }

    #[test]
    fn test_parse() {
        let cases: Vec<(&str, Result<Expression, &str>)> = vec![
            ("2d4", Ok(Sum(Box::new(Die(4)), Box::new(Die(4))))),
            // trailing whitespace is ok
            (" \t\nd10 \t\n", Ok(Die(10))),
            // entire unknown keyword
            ("2d4 extra", Err("")),
            // must include operator between base terms
            ("d4 d4", Err("")),
            // only one comparison operator allowed
            ("d4 > d6 < d10", Err("")),
        ];

        for (input, expected) in cases {
            match parse(input) {
                Ok(actual) => assert_eq!(actual, expected.unwrap()),
                Err(_) => assert!(expected.is_err()),
            }
        }
    }
}
