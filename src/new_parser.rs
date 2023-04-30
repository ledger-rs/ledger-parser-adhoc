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

/// Entry point.
/// A File or a Cursor (for text) can be passed in to be parsed.
pub fn parse<T: Read>(source: T) {
    // reader
    let mut reader = BufReader::new(source);
    // for line_result in reader.lines() {

    // To avoid allocation, read line by line.
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
                print!("read {:?} bytes, ", num_bytes);
                println!("content: {:?}", line);

                // Remove the trailing newline characters
                // line.pop()
                // TODO: line.strip_suffix(suffix)
                let clean_line = strip_trailing_newline(&line);

                // use the read value
                read_next_directive(&clean_line);

                // clear the buffer before reading the next line.
                line.clear();
            }
        }
    }
}

/// textual.cc
/// void instance_t::read_next_directive(bool &error_flag)
fn read_next_directive(line: &str) {
    let len = line.len();
    if len == 0 {
        return;
    }

    let first_char = line.chars().nth(0).expect("first character");
    match first_char {
        // comments
        ';' | '#' | '*' | '|' => {
            // ignore
        }

        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => xact_directive(line),

        _ => {
            // if !general_directive()
            // the rest
            todo!("the rest")
        }
    }
}

fn xact_directive(line: &str) {
    let xact = parse_xact(line);

    todo!("add xact to journal");
}

fn parse_xact(line: &str) -> Xact {
    let (next_start, next_end) = next_element(line);

    // Parse the date
    let date = parse_date(line);

    if line.contains('=') {
        // TODO: Parse the aux date
    }

    // TODO: Parse the optional cleared flag: *

    // Parse the optional code: (TEXT)
    // TODO: Parse the description text
    let payee = "";
    // TODO: Parse the xact note
    let note = "";
    // TODO: Parse all of the posts associated with this xact
    // Tags

    Xact::new(Some(date), payee, Some(note))
}

/// Finds the boundaries of the next text element.
fn next_element(line: &str) -> usize {
    let location: usize;

    // iterate over the string
    for p in line.chars() {

    }

    location
}

fn parse_date(line: &str) -> NaiveDate {
    // It should be enough to get the content until the first whitespace.
    let ws_index = line.find(' ').expect("date end");
    let date_str = &line[0..ws_index];

    // todo: support more date formats?
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .expect("date parsed");

    date
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

#[cfg(test)]
mod tests {
    use crate::new_parser::parse;

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

        parse(tx_str.as_bytes());

        assert!(false);
    }
}
