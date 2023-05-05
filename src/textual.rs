use std::{
    fs::File,
    io::BufReader,
};

use crate::{
    context::{ParseContext},
    xact::Xact,
};

/**
 * textual.cc
 */

pub(crate) struct Instance<'a> {
    pub context_stack: &'a Vec<ParseContext<'a>>,
    pub context: &'a ParseContext<'a>,
    pub reader: BufReader<File>,
    //pub apply_stack: Vec<Application>
}

impl<'a> Instance<'a> {
    pub fn new(context_stack: &Vec<ParseContext>, context: &ParseContext) -> Self {
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
        // '\0' => println!("zero"),

        ' ' | '\t' => {
            if !error_flag {
                panic!("Unexpected whitespace at beginning of line");
            }
        }

        // comments
        ';' | '#' | '*' | '|' => {
            // ignore
        }

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
        }

        _ => {
            // if !general_directive()
            // the rest
            todo!("the rest")
        }
    };

    println!("complete");
}

fn xact_directive(line: &str, len: usize) {
    // let xact = parse_xact(line, len, top_account());
    let xact = parse_xact(line);

    // context.journal.add_xact(xact);

    todo!("complete")
}

fn parse_xact(line: &str) {
    // let mut xact = Xact::new("".into(), None);

    // date

    let next = next_element(line);
    // todo: skip whitespace

    // Locate first occurrence of character in string
    //let next = line.find('=');

    todo!("complete")
}

/// utils.h
/// inline char * next_element(char * buf, bool variable = false)
/// Get the next element to work with.
fn next_element(line: &str) -> usize {
    // variable = false
    for p in line.chars() {
        if !(p == ' ' || p == '\t') {
            continue;
        }

        // todo: if !variable {
        //return skip_ws(line);
        // }
    }

    todo!("complete")
}

fn skip_ws(line: &str, pos: usize) -> usize {
    // let start = pos;
    // let end = line.len();

    // for p in [start..end] {
    //     println!("yo {:?}", p);
    // }

    // nth() consumes characters
    // while let char = line.chars().nth(0) {
    //     println!("char: {:?}", char);
    // }

    // https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings

    // line.chars();

    todo!("complete")
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

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
    fn test_char_iteration() {
        let string = "Hello";
        let mut iterator = string.chars();

        let third = iterator.nth(2);
        println!("third: {:?}", third);
        let second = iterator.nth(1);
        println!("second: {:?}", second);

        println!("second iterator, third = {:?}", string.chars().nth(2));
        
        println!("string after nth: {:?}", string);
        println!("chars: {:?}", string.chars());
    }
}
