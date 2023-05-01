/**
 * This parser is created from scratch, using Rust's concepts, instead of trying to
 * map C++ constructs onto Rust.
 * Issues with pointers for the model structure, as well as parsing files and strings
 * seem beter suited to be re-implemented anew.
 */
// todo: get the list of files to parse

// todo: generate a stream (or Read?), which can be text or File.
use std::io::{BufRead, BufReader, Read};

use chrono::NaiveDate;

use crate::xact::Xact;

struct NewParser {
    is_parsing_xact: bool,
    xact: Option<Xact>,
}

impl NewParser {
    pub fn new() -> Self {
        Self { is_parsing_xact: false, xact: None }
    }

    /// Entry point.
    /// A File or a Cursor (for text) can be passed in to be parsed.
    pub fn parse<T: Read>(&mut self, source: T) {
        // reader
        let mut reader = BufReader::new(source);
        // for line_result in reader.lines() {

        // To avoid allocation, reuse the String variable.
        let mut line = String::new();
        loop {
            match reader.read_line(&mut line) {
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
                Ok(0) => {
                    // end of file?
                    println!("End of file");
                    break;
                }
                Ok(num_bytes) => {
                    // print!("read {:?} bytes, ", num_bytes);
                    // println!("content: {:?}", line);

                    // Remove the trailing newline characters
                    // let clean_line = strip_trailing_newline(&line);
                    let clean_line = &line.trim_end();

                    // use the read value
                    self.parse_line(&clean_line);

                    // clear the buffer before reading the next line.
                    line.clear();
                }
            }
        }

        todo!("Return the result")
    }

    /// textual.cc
    /// void instance_t::read_next_directive(bool &error_flag)
    fn parse_line(&mut self, line: &str) {
        let len = line.len();
        if len == 0 {
            if self.is_parsing_xact {
                // An empty line is a separator between transactions.
                self.is_parsing_xact = false;
                todo!("finalize the transaction")
            }

            return;
        }

        let first_char = line.chars().nth(0).expect("first character");
        match first_char {
            // comments
            ';' | '#' | '*' | '|' => {
                // ignore
            }

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.xact_directive(line)
            }

            ' ' | '\t' => {
                if self.is_parsing_xact {
                    todo!("parse posting")
                } else {
                    panic!("Unexpected whitespace at beginning of line");
                }
            }

            _ => {
                // if !general_directive()
                // the rest
                todo!("the rest")
            }
        }
    }

    fn xact_directive(&mut self, line: &str) {
        let xact = parse_xact(line);
    
        self.xact = Some(xact);
    
        todo!("add xact to journal");
    }
    
}

fn parse_xact(line: &str) -> Xact {
    // let mut next_start: usize = 0;
    let next = next_element(line, 0);
    let mut next_index = match next {
        Some(index) => index,
        None => 0,
    };

    // Parse the date
    let date = parse_date(line);

    if line.contains('=') {
        // TODO Parse the aux date
    }

    // TODO Parse the optional cleared flag: *
    // if next.is_some() {}

    // Parse the optional code: (TEXT)
    // if next.is_some() && next == '(' {}

    // Parse the description text
    let mut payee = "<Unspecified payee>".to_owned();
    if next.is_some() && next_index < line.len() {
        let mut pos = next_index;
        let mut spaces: usize = 0;
        let mut tabs: usize = 0;
        // iterate further
        for character in line.chars().skip(next_index) {
            if character == ' ' {
                spaces += 1;
            } else if character == '\t' {
                tabs += 1;
            } else if character == ';' && (tabs > 0 || spaces > 1) {
                todo!("complete")
            } else {
                spaces = 0;
                tabs = 0;
            }
            pos += 1;
        }
        // TODO: validate payee
        // xact->payee = context.journal->validate_payee(next);
        payee = line[next_index..].into();
        // next = p;
        next_index = pos;
    }

    // Parse the xact note
    let note = "".to_string();
    if next.is_some()
        && next_index < line.len()
        && line.chars().skip(next_index).next() == Some(';')
    {
        todo!("append note")
    }

    // TODO: Parse all of the posts associated with this xact

    // Tags

    Xact::new(Some(date), payee, Some(note))
}

/// Finds the start of the next text element.
/// utils.h
/// inline char * next_element(char * buf, bool variable = false)
fn next_element(line: &str, start: usize) -> Option<usize> {
    let mut position: usize = 0;

    // iterate over the string
    for p in line.char_indices().skip(start) {
        let character = p.1;
        if !(character == ' ' || character == '\t') {
            continue;
        }

        // if !variable {
        position = p.0 + 1;
        return skip_ws(line, &position);
        // }
    }

    None
}

fn parse_date(line: &str) -> NaiveDate {
    // It should be enough to get the content until the first whitespace.
    let ws_index = line.find(' ').expect("date end");
    let date_str = &line[0..ws_index];

    // todo: support more date formats?
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").expect("date parsed");

    date
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

/// Starts iterating through the string at the given location,
/// skips the whitespace and returns the location of the next element.
fn skip_ws(line: &str, start: &usize) -> Option<usize> {
    for p in line.char_indices().skip(*start) {
        let character = p.1;
        while character == ' ' || character == '\t' || character == '\n' {
            continue;
        }
        return Some(p.0);
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::new_parser::{next_element, NewParser};

    #[test]
    fn basic_tx_test() {
        // a simple transaction with space on top.
        let tx_str = r#"
2023-04-10 Supermarket
    Expenses:Food  20 EUR
    Assets:Cash
"#;

        // Cursor already implements BufReader!
        //let cursor = Cursor::new(tx_str);

        let mut parser = NewParser::new();
        parser.parse(tx_str.as_bytes());

        assert!(false);
    }

    #[test]
    fn test_finding_next_element() {
        let line = "2023-01-12 Supermarket";

        let actual = next_element(line, usize::MIN);

        // The next item (Supermarket) starts at index 11.
        assert_eq!(Some(11), actual);
    }
}
