use std::{
    env,
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use crate::{account::Account, journal::Journal};

/**
 * context.h
 */

const MAX_LINE: usize = 4096;

pub(crate) struct ParseContext<'path> {
    // stream
    pub pathname: &'path PathBuf,
    pub current_directory: PathBuf,
    pub journal: &'path Journal,
    pub master: &'path Account,
    // pub scope: Scope
    // linebuf
    // line_beg_pos
    // curr_pos
    // linenum
    // errors
    // count
    // sequence
    // last
}

impl<'path> ParseContext<'path> {
    pub fn new(
        pathname: &'path PathBuf,
        cwd: PathBuf,
        journal: &'path Journal,
        master: &'path Account,
    ) -> Self {
        ParseContext {
            pathname,
            current_directory: cwd,
            journal,
            master,
        }
    }

    pub fn open_for_reading(pathname: &PathBuf, cwd: &PathBuf) -> Self {
        // todo: get absolute filename

        // Below is the example of efficient reading line-by-line:

        // let file = File::open(pathname).unwrap();
        // let buf_reader = BufReader::new(file);
        // file.read(buf)
        // for line in reader.lines() {
        //     println!("{}", line?);
        // }

        todo!()
    }
}

pub(crate) struct ParseContextStack<'path> {
    parsing_context: Vec<ParseContext<'path>>,
}

impl<'path> ParseContextStack<'path> {
    pub fn new() -> Self {
        Self {
            parsing_context: vec![],
        }
    }

    /// Replaces the call to .push, when adding a new context.
    pub fn add_new(&mut self, pathname: &'path PathBuf, journal: &'path Journal, master: &'path Account) {
        let cwd = env::current_dir().unwrap();
        let context = ParseContext::new(pathname, cwd, journal, master);
        self.parsing_context.push(context);
    }

    pub fn get_current(&self) -> &ParseContext {
        assert!(!self.parsing_context.is_empty());

        self.parsing_context.last().unwrap()
    }

    pub fn push(&mut self, pathname: &PathBuf) {
        let cwd = env::current_dir().unwrap();
        // let mut new_context = ParseContext::open_for_reading(&pathname, &cwd);
        // let new_context = ParseContext::new(pathname, &cwd)

        // self.parsing_context.push(new_context);
        todo!("check, this may not be required after add_new was added")
    }
}
