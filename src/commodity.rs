use crate::utils::peek_next_nonws;


#[derive(Debug, PartialEq)]
pub(crate) struct Commodity {
    symbol: String,
    // graph_index
    // precision
    // name
    // note
    // smaller
    // larger
    // value_expr
}

impl Commodity {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

/// Parse symbol from the input string.
/// Original code in
/// void commodity_t::parse_symbol(std::istream& in, string& symbol)
pub fn parse_symbol(input: &str) -> &str {
    // skip ws
    let c = peek_next_nonws(input);

    // symbols in quotes
    if input.chars().skip(c).next() == Some('\"') {
        todo!("read everything until the closing quote")
    } else {
        // todo invalid characters? Does Rust have the same limitation?
        
        //let buf = &input[c..];
        // is_reserved_token


    }

    &input[c..]
}

#[cfg(test)]
mod tests {
    use super::parse_symbol;

    #[test]
    fn test_parse_symbol() {
        let actual = parse_symbol(" EUR");

        assert_eq!("EUR", actual);
    }

    #[test]
    fn test_parse_symbol_not_allowed_in_ledger() {
        let actual = parse_symbol(" EL4X");

        assert_eq!("EL4X", actual);
    }

}