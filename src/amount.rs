use crate::commodity::Commodity;

/**
 * amount.h + amount.cc
 */

#[derive(Debug, PartialEq)]
pub struct Amount {
    precision: u16,

    quantity: i64,
    commodity: Commodity,
}

impl Amount {
    pub fn new() -> Self {
        Self {
            precision: 0,
            quantity: 0,
            commodity: Commodity::new(),
        }
    }

    /// Parse amount
    /// amount.cc
    /// bool amount_t::parse(std::istream& in, const parse_flags_t& flags)
    /// The possible syntax for an amount is:
    ///   [-]NUM[ ]SYM [@ AMOUNT]
    ///   SYM[ ][-]NUM [@ AMOUNT]
    pub fn parse(input: &str) -> Amount {
        let symbol: &str;
        let quant: &str;
        // details
        let mut negative = false;

        let mut c = peek_next_nonws(input, 0);
        let next_char = input.chars().skip(c).next();

        if next_char == Some('-') {
            negative = true;
            c = peek_next_nonws(input, c);
        }

        if next_char.unwrap().is_digit(10) {
            let num_slice = &input[0..];
            quant = parse_quantity(num_slice);

            // COMMODITY_STYLE_SEPARATED

            //let symbol_slice = &line[]
            symbol = parse_symbol(input);
        }

        todo!("parse amount")
    }
}

/// Find the next non-whitespace character location.
fn peek_next_nonws(line: &str, start: usize) -> usize {
    for (i, c) in line.char_indices().skip(start) {
        if c == ' ' {
            continue;
        } else {
            // got to the first space
            return i;
        }
    }
    return 0;
}

/// Returns the str of the quantity value.
fn parse_quantity(input: &str) -> &str {
    let mut start: usize = 0;
    let mut end: usize = 0;

    if input.chars().next() == Some('-') {
        start += 1;
    } 

    // read characters so long as they are numeric.
    for (i, c) in input.char_indices().skip(start) {
        if c.is_digit(10) || c == '.' || c == ',' {
            continue;
        } else {
            return &input[start..i];
        }
    }
    return "";
}

fn parse_symbol(input: &str) -> &str {
    todo!("parse symbol")
}

#[cfg(test)]
mod tests {
    use crate::amount::{parse_quantity, parse_symbol};

    use super::{peek_next_nonws, Amount};

    #[test]
    fn test_amount_parsing_20_eur() {
        let amount_str = "20 EUR";
        let expected = Amount::new();

        let actual = Amount::parse(amount_str);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_amount_parsing_negative() {
        let amount_str = "-20 EUR";
        let expected = Amount::new();

        let actual = Amount::parse(amount_str);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_peek_next_nonws() {
        let line = "10 ABC";

        let actual = peek_next_nonws(line, 0);

        assert_eq!(0, actual);
    }

    #[test]
    fn test_peek_next_nonws_without_space() {
        let line = "10ABC";

        let actual = peek_next_nonws(line, 0);

        assert_eq!(0, actual);
    }

    #[test]
    fn test_parse_quantity_positive() {
        let actual = parse_quantity("20 EUR");

        assert_eq!("20", actual)
    }

    #[test]
    fn test_parse_quantity_negative() {
        let actual = parse_quantity("-20 EUR");

        assert_eq!("20", actual);
    }

    #[test]
    fn test_parse_symbol() {
        let input = "20 EUR";
        let actual = parse_symbol(input);

        assert_eq!("EUR", actual);
    }
}
