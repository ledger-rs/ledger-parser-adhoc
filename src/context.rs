use std::path::{Path, PathBuf};

use crate::journal::Journal;

/**
 * context.h
 */

pub(crate) struct ParseContext {
    pathname: PathBuf,
    current_directory: PathBuf,
    journal: Journal
}

pub(crate) struct ParseContextStack {
    parsing_context: Vec<ParseContext>
}

impl ParseContextStack {
    pub fn new() -> Self {
        Self { parsing_context: vec![]  }
    }
}