use nom;
use nom::digit;

use std::str;

use crate::expression::Expression;
use Expression::*;

// #[derive(Debug, PartialEq)]
// pub struct Color {
//     pub red: u8,
//     pub green: u8,
//     pub blue: u8,
// }

// pub fn from_str(input: &str) -> Result<i32, std::num::ParseIntError> {
//     i32::from_str_radix(input, 10)
// }

// pub fn is_decimal_digit(c: char) -> bool {
//     // println!("{}.is_digit(10) = {}", c, c.is_digit(10));
//     c.is_digit(10)
// }

// pub fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
//     u8::from_str_radix(input, 16)
// }

// pub fn is_hex_digit(c: char) -> bool {
//     c.is_digit(16)
// }

// TODO: parse basic die roll

// named!(die<&str,&Expression>,nom::i32)

// named!(hex_primary<&str, u8>,
//     map_res!(take_while_m_n!(2, 2, is_hex_digit), from_hex)
// );

// named!(hex_color<&str, Color>,
//     do_parse!(
//                tag!("#")   >>
//         red:   hex_primary >>
//         green: hex_primary >>
//         blue:  hex_primary >>
//         (Color { red, green, blue })
//     )
// );
// named!(pub parse_number<&str,i32>,map_res!( take_while!(is_digit),nom::be_i32));
named!(take4<&str,&str>, take!(4));
named!(num<&str, i32>, map_res!(ws!(digit), parse_i32));

fn parse_i32(input: &str) -> Result<i32, std::num::ParseIntError> {
    // println!(
    //     "int32::from_str_radix({}, 10) = {:?}",
    //     input,
    //     i32::from_str_radix(input, 10)
    // );

    i32::from_str_radix(input, 10)
}

named!(
    ptag,
    tap!(res: tag!( "abcd" ) => { println!("HELLO!");
    println!("\n\nrecognized {}\n\n", str::from_utf8(res).unwrap()) } )
);

// named!(parse_number< &str, i32 >, map_res!(take_while!(is_decimal_digit), parse_i32));

// fn parse_num(num: &str) -> Result<i32, std::num::ParseIntError> {
//     i32::from_str_radix(num, 10)
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_ptag() {
    //     assert_eq!(Ok((&b"efg"[..], &b"abcd"[..])), ptag(&b"abcdefg"[..]));
    // }

    #[test]
    fn test_parse_number() {
        assert_eq!(Ok(1234), parse_i32("1234"));
        assert_eq!(Ok(("", 1234)), num("1234"));
    }

    // #[test]
    // fn test_take4() {
    //     assert_eq!(Ok(("56789", "1234")), take4("123456789"));
    // }

    // #[test]
    // fn test_parse_num

    // #[test]
    // fn parse_color() {
    //     assert_eq!(
    //         hex_color("#2F14DF"),
    //         Ok((
    //             "",
    //             Color {
    //                 red: 47,
    //                 green: 20,
    //                 blue: 223,
    //             }
    //         ))
    //     );
    // }
}
