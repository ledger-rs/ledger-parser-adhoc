/**
 * session.cc + session.h
 *
 * Session objects
 * Every journal object is owned by a session, with the session providing support for that object.
 * In GUI terms, this is the Controller object for the journal Data object, where every document
 * window would be a separate session. They are all owned by the global scope.
 */
use std::{
    io::stdin,
    path::{self, PathBuf},
};

use crate::{account::Account, context::ParseContext, journal::Journal};

pub(crate) struct Session<'a> {
    pub data_files: Vec<PathBuf>,

    pub journal: Journal,
    pub parse_context_stack: Vec<ParseContext<'a>>,
}

impl<'a> Session<'a> {
    pub fn new() -> Self {
        Self {
            journal: Journal::new(),
            data_files: vec![],
            parse_context_stack: vec!(),
        }
    }

    pub(crate) fn read_journal_files(&'a mut self) -> &Journal {
        log::info!("Read journal file");

        let master_account: String = "".into();

        let count: usize = self.read_data(master_account);

        todo!("&self.journal")
    }

    ///session.cc, line 72
    fn read_data(&'a mut self, master_account: String) -> usize {
        let populated_data_files = false;
        if self.data_files.is_empty() {
            // todo: get file paths
        }

        let mut xact_count: usize = 0;

        let account: &Account = if master_account.is_empty() {
            &self.journal.master
        } else {
            // todo: check what the default parameter here is!
            &self.journal.find_account(&master_account, true)
        };

        // todo: price_db
        // let price_db_path: PathBuf;
        // todo: day_break
        // todo: recursive_aliases
        // ...
        // check_payees

        // todo: load price db
        // if price_db_path...

        for pathname in self.data_files.iter() {
            if pathname.to_str() == Some("-") || pathname.to_str() == Some("/dev/stdin") {
                // TODO: read from stdin
                let buffer = stdin().lines();
            } else {
                let parse_context = ParseContext::new(pathname, &self.journal, account);
                self.parse_context_stack.push(parse_context);
            }

            // todo: set journal
            // self.parsing_context.get_current().journal = self.journal.get();
            let journal = &self.journal;
            // todo: set master
            // self.parsing_context.get_current().ma
            let parse_context = ParseContext::new(pathname, journal, account);
            self.parse_context_stack.push(parse_context);

            xact_count += self.journal.read(&self.parse_context_stack);

            // todo: remove from the parsing context
        }

        xact_count
    }

    fn read_journal(&self) {}
}

/// TODO: Create an entry point that is called from code, not CLI.
/// Run a report
pub fn run(args: Vec<String>) {
    // todo: read journal files (data)

    // todo: run report
}

#[cfg(test)]
mod tests {
    use crate::session::Session;

    #[test]
    fn test_basic() {
        let tx = r#"
2023-04-10 Supermarket
    Expenses:Food  20 EUR
    Assets:Cash

"#;
        let mut s = Session::new();
        let actual = s.read_journal_files();

        assert!(false);
        // assert_eq!(1, actual);
    }

    /// development test, used for debugging through.
    #[test]
    fn test_bal_report() {
        let args: Vec<String> = vec!["b".into()];

        // let actual = run(args);

        assert!(false)
    }

    // Try to use this simplest form for debugging.
    #[test]
    fn test_most_basic_balance_report() {
        // no commodities; simple account structure
        let tx = r#"
2023-04-10 Supermarket
    Expenses  20
    Assets

"#;
        let session = Session::new();

        // session.

        // let actual =

        todo!("complete")
    }
}
