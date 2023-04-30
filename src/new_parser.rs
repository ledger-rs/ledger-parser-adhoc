/**
 * This parser is created from scratch, using Rust's concepts, instead of trying to
 * map C++ constructs onto Rust.
 * Issues with pointers for the model structure, as well as parsing files and strings
 * seem beter suited to be re-implemented anew.
 */
// todo: get the list of files to parse

// todo: generate a stream (or Read?), which can be text or File.
use std::io::{BufRead, BufReader, Read};

/// Entry point.
/// A File or a Cursor (for text) can be passed in to be parsed.
fn parse<T: Read>(source: T) {
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

                // todo: use the read value

                // clear the buffer before reading the next line.
                line.clear();
            }
        }
    }
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
