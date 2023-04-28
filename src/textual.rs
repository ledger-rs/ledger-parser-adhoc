use std::{io::BufReader, fs::File};

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
            reader: todo!()
        }
    }

    pub fn parse(&self) -> usize {
        log::info!("Parsing {:?}", self.context.pathname);

        // self.context.linenum = 0;

        self.read_next_directive();

        todo!()
    }

    fn read_next_directive(&self) {
        self.read_line()
    }

    fn read_line(&self) {

    }
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
    fn test_basic() {
        let tx_str = r#"
2023-04-10 Supermarket
    Expenses:Food  20 EUR
    Assets:Cash
"#;

        todo!("parse text")
    }

}
