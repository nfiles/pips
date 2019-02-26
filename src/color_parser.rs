#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

pub fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

named!(hex_primary<&str, u8>,
    map_res!(take_while_m_n!(2, 2, is_hex_digit), from_hex)
);

named!(hex_color<&str, Color>,
    do_parse!(
               tag!("#")   >>
        red:   hex_primary >>
        green: hex_primary >>
        blue:  hex_primary >>
        (Color { red, green, blue })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_color() {
        assert_eq!(
            hex_color("#2F14DF"),
            Ok((
                "",
                Color {
                    red: 47,
                    green: 20,
                    blue: 223,
                }
            ))
        );
    }
}
