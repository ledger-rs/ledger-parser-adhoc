use crate::{commodity::{Commodity, self}, utils};

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

        // cursor for chars.
        let mut c = utils::peek_next_nonws(input);
        let mut next_char = input.chars().skip(c).next();

        if next_char == Some('-') {
            // TODO: complete the negative number parsing.
            negative = true;
            c = utils::peek_next_nonws(input);
            next_char = input.chars().skip(c).next();
        }

        if next_char.unwrap().is_digit(10) {
            let offset: usize;
            (quant, offset) = parse_quantity(input);
            // move the cursor
            c += offset;

            // COMMODITY_STYLE_SEPARATED

            symbol = commodity::parse_symbol(&input[c..]);
        }

        todo!("parse amount")
    }
}

/// Identifies the quantity in the input string.
/// Returns the str of the quantity value and
/// the last position index (for cursor control).
/// Parameters:
/// input
fn parse_quantity(input: &str) -> (&str, usize) {
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
            return (&input[start..i], i);
        }
    }
    return ("", 0);
}


#[cfg(test)]
mod tests {
    use crate::{amount::parse_quantity, utils::peek_next_nonws, commodity::parse_symbol};

    use super::Amount;

    #[test]
    fn test_parse_20_eur() {
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

        let actual = peek_next_nonws(line);

        assert_eq!(0, actual);
    }

    #[test]
    fn test_peek_next_nonws_without_space() {
        let line = "10ABC";

        let actual = peek_next_nonws(line);

        assert_eq!(0, actual);
    }

    #[test]
    fn test_parse_quantity_positive() {
        let (actual, offset) = parse_quantity("20 EUR");

        assert_eq!("20", actual);
        assert_eq!(2, offset);
    }

    #[test]
    fn test_parse_quantity_negative() {
        let (actual, offset) = parse_quantity("-20 EUR");

        assert_eq!("20", actual);
        assert_eq!(3, offset);
    }

    #[test]
    fn test_parse_symbol() {
        let input = "20 EUR";
        let actual = parse_symbol(input);

        assert_eq!("EUR", actual);
    }
}
