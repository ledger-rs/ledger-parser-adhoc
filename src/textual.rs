use std::{fs::File, io::{BufReader, Read, BufRead}};

use crate::context::{ParseContext, ParseContextStack};

/**
 * textual.cc
 */

pub(crate) struct Instance<'a> {
    pub context_stack: &'a ParseContextStack<'a>,
    pub context: &'a ParseContext<'a>,
    pub reader: BufReader<File>,
    //pub apply_stack: Vec<Application>
}

impl<'a> Instance<'a> {
    pub fn new(context_stack: &'a ParseContextStack<'a>, context: &'a ParseContext<'a>) -> Self {
        Self {
            context_stack,
            context,
            //reader: context.stream.get(),
            reader: todo!(),
        }
    }

    pub fn parse(&self) -> usize {
        log::info!("Parsing {:?}", self.context.pathname);

        // self.context.linenum = 0;

        self.read_next_directive();

        todo!()
    }

    fn read_next_directive(&self) {
        // todo: manage reading lines from files here.
        // todo: let line = self.read_line();
        let line = "some line";

        // continuing in...
        process_line(line);
    }

    /// textual.cc
    /// std::streamsize instance_t::read_line(char *&line)
    fn read_line(&self) {
        // this reads a line into the passed pointer's variable
        // and returns the length.
        // In Rust, however, the pointer magic is not practical.
    }
}

fn read_source<T: Read>(source: T) {
    // reader
    let reader = BufReader::new(source);
    for line_result in reader.lines() {
        let line = line_result.expect("line read");
        // println!("line: {}", line);

        process_line(&line);
    }
}

/// Extracting the line processing logic here as the order of reading/processing
/// will most-likely change in Rust, compared to the logic in C++.
fn process_line(line: &str) {
    let len = line.len();
    if len == 0 {
        return;
    }

    let first_char = line.chars().nth(0).expect("the first character");
    let error_flag = !first_char.is_whitespace();

    match first_char {
        '\0' => println!("zero"),

        ' ' | '\t' => {
            if !error_flag {
                panic!("Unexpected whitespace at beginning of line");
            }
        },

        // comments
        ';' | '#' | '*' | '|' => {
            // ignore
        },

        // option setting
        '-' => {
            todo!("option setting")
        }

        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => xact_directive(line, len),

        // automated xact
        '=' => todo!("automated xact"),

        // period xact
        '~' => todo!("period xact"),

        '@' | '!' => {
            // fall through, line++
            todo!("increase the line then go down the default branch")
        },

        _ => {
            // if !general_directive()
            // the rest
            todo!("the rest")
        },
    };

    println!("complete");
}

fn xact_directive(line: &str, len: usize) {
    // let xact = parse_xact(line, len, top_account());

    // context.journal.add_xact(xact);
    
    todo!("complete")
}

fn parse_xact() {
    todo!("complete")
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Cursor},
    };

    use crate::textual::read_source;

    /// Below is the example of efficient reading line-by-line
    #[test]
    fn read_file_example() {
        let pathname = "tests/first.ledger";

        let file = File::open(pathname).unwrap();
        let reader = BufReader::new(file);
        //file.read(buf_reader);
        for line in reader.lines() {
            println!("{}", line.expect("line read"));
        }
    }

    #[test]
    fn basic_tx_test() {
        let tx_str = r#"
2023-04-10 Supermarket
    Expenses:Food  20 EUR
    Assets:Cash
"#;

        let cursor = Cursor::new(tx_str);
        read_source(cursor);

        todo!("parse text")
    }
}
