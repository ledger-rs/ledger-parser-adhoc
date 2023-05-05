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

pub struct ParseContext<'a> {
    // stream
    pub pathname: &'a PathBuf,
    // pub current_directory: PathBuf,
    pub journal: &'a Journal,
    pub master: &'a Account,
    // pub scope: Scope
    // linebuf
    // line_beg_pos
    // curr_pos
    // pub linenum: usize,
    pub errors: usize,
    pub count: usize,
    // pub sequence: usize,
    // pub last: String
}

impl<'a> ParseContext<'a> {
    pub fn new(
        pathname: &'a PathBuf,
        // cwd: PathBuf,
        journal: &'a Journal,
        master: &'a Account,
    ) -> Self {
        ParseContext {
            pathname,
            // current_directory: cwd,
            journal,
            master,
            // linenum: 0,
            errors: 0,
            count: 0,
            // sequence: 0,
            // last: "".to_string(),
        }
    }

    // pub fn from_pathname(pathname: &PathBuf) -> Self {
    //     Self::new(pathname, &Journal::new(), &Account::new())
    // }

    pub fn open_for_reading(pathname: &PathBuf, cwd: &PathBuf) -> Self {
        // todo: get absolute filename

        todo!()
    }
}

// pub struct ParseContextStack<'a> {
//     parsing_context: Vec<ParseContext<'a>>,
// }

// impl<'a> ParseContextStack<'a> {
//     pub fn new() -> Self {
//         Self {
//             parsing_context: vec![],
//         }
//     }

//     /// Replaces the call to .push, when adding a new context.
//     pub fn add_new(&mut self, pathname: &'a PathBuf, journal: &'a Journal, master: &'a Account) {
//         // let cwd = env::current_dir().unwrap();
//         let context = ParseContext::new(pathname, journal, master);
//         self.parsing_context.push(context);
//     }

//     pub fn get_current(&self) -> &ParseContext {
//         assert!(!self.parsing_context.is_empty());

//         self.parsing_context.last().unwrap()
//     }

//     pub fn push(&mut self, pathname: &PathBuf) {
//         let cwd = env::current_dir().unwrap();
//         // let mut new_context = ParseContext::open_for_reading(&pathname, &cwd);
//         // let new_context = ParseContext::new(pathname, &cwd)

//         // self.parsing_context.push(new_context);
//         todo!("check, this may not be required after add_new was added")
//     }
// }
