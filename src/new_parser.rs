/**
 * This parser is created from scratch, using Rust's concepts, instead of trying to 
 * map C++ constructs onto Rust.
 * Issues with pointers for the model structure, as well as parsing files and strings
 * seem beter suited to be re-implemented anew.
 */

// todo: get the list of files to parse

// todo: generate a stream (or Read?), which can be text or File.

use std::io::{Read, BufReader, BufRead};

/// Entry point.
/// A File or a Cursor (for text) can be passed in to be parsed.
fn parse<T: Read>(source: T) {
    // reader
    let mut reader = BufReader::new(source);
    // for line_result in reader.lines() {
    //     let line = line_result.expect("line read");

    //     //process_line(&line);
    // }

    // To avoid allocation, read line by line.
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(num_bytes) => {
            println!("read {:?} bytes.", num_bytes);
            println!("content: {:?}", line);
        },
        Err(err) => {
            // end of file?
            println!("error: {:?}", err);
        },
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

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