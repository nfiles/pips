use nom;
use nom::digit;
use nom::types::CompleteStr;

use std::str::FromStr;

use crate::expression::Expression;
use Expression::*;

// named!(
//     parse_signs<CompleteStr, CompleteStr>,
//     ws!(take_while!(call!(|c| c == '+' || c == '-')))
// );

named!(
    parse_eval_signs<CompleteStr, char>,
    map!(
        ws!(take_while!(call!(|c| c == '+' || c == '-'))),
        |input: CompleteStr| {
            let neg_count = input.chars().filter(|&x| x == '-').count();
            match neg_count % 2 {
                1 => '-',
                _ => '+',
            }
        }
    )
);

named!(
    pub parse_unsigned_number<CompleteStr, u32>,
    map_res!(
        recognize!(digit),
        |CompleteStr(string)| u32::from_str(string)
    )
);

named!(
    pub parse_signed_number<CompleteStr, i32>,
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
    pub parse_constant<CompleteStr, Expression>,
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
    pub parse_die<CompleteStr, Expression>,
    alt_complete!(parse_die_single)
    // alt_complete!(parse_die_single | parse_die_coefficient)
);
