use crate::commodity::Commodity;

/**
 * amount.h + amount.cc
 */

 #[derive(Debug, PartialEq)]
pub struct Amount {
    precision: u16,

    quantity: i64,
    commodity: Commodity
}

impl Amount {
    pub fn new() -> Self {
        Self { precision: 0, quantity: 0, commodity: Commodity::new() }
    }

    pub fn parse(line: &str, pos: usize) -> Amount {
        let x = peek_next_nonws(line, pos);
        todo!("parse amount")
    }
}

fn peek_next_nonws(line: &str, start: usize) -> usize {
    for (i, c) in line.char_indices().skip(start) {
        if c != ' ' {
            continue;
        } else {
            // got to the first space
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{Amount, peek_next_nonws};

    #[test]
    fn test_parsing_20_EUR() {
        let amount_str = "20 EUR";
        let expected = Amount::new();

        let actual = Amount::parse(amount_str, 0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_peek_next_nonws() {
        let line = "10 ABC";

        let actual = peek_next_nonws(line, 0);

        assert_eq!(2, actual);
    }

    #[test]
    fn test_peek_next_nonws_without_space() {
        let line = "10ABC";

        let actual = peek_next_nonws(line, 0);

        assert_eq!(0, actual);
    }

}