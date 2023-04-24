use std::path::Path;

use crate::journal::Journal;

/**
 * context.h
 */

struct ParseContext {
    pathname: Box<Path>,
    current_directory: Box<Path>,
    journal: Journal
}