use crate::context::{ParseContext, ParseContextStack};

/**
 * textual.cc
 */

pub(crate) struct Instance<'a> {
    pub context_stack: &'a ParseContextStack<'a>,
    pub context: &'a ParseContext<'a>,
    //pub apply_stack: Vec<Application>
}

impl<'a> Instance<'a> {
    pub fn new(context_stack: &'a ParseContextStack<'a>, context: &'a ParseContext<'a>) -> Self {
        Self {
            context_stack,
            context,
        }
    }

    pub fn parse(&mut self) -> usize {
        log::info!("Parsing {:?}", self.context.pathname);

        // self.context.linenum = 0;

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Read},
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

}
