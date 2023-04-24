use std::path::{Path, PathBuf};

use crate::{account::Account, context::{ParseContext, ParseContextStack}, journal::Journal};

/**
 * session.cc + session.h
 */

pub(crate) struct Session {
    pub data_files: Vec<PathBuf>,

    pub journal: Journal,
    pub parsing_context: ParseContextStack,
}

impl Session {
    pub fn new() -> Self {
        Self {
            journal: Journal::new(),
            data_files: vec![],
            parsing_context: ParseContextStack::new(),
        }
    }

    pub(crate) fn read_journal_files(&self) -> &Journal {
        log::info!("Read journal file");

        let master_account: &str = "";

        let count: usize = self.read_data(master_account);

        &self.journal
    }

    ///session.cc, line 72
    fn read_data(mut self, master_account: &str) -> usize {
        // todo: get file paths

        let mut xact_count: usize = 0;

        // let account: Account = if master_account.is_empty() {
        //     self.journal.master
        // } else {
        //     self.journal.find_account(master_account)
        // };
        let account = &self.journal.master;

        // price_db
        // ...
        // check_payees

        // todo: load price db

        for pathname in self.data_files.iter() {
            // todo: handle - and stdin

            // todo: self.parsing_context.append(pathname);

            // set journal
            // set master

            xact_count += self.journal.read(&self.parsing_context);

            // todo: remove from the parsing context
        }

        xact_count
    }

    fn read_journal(&self) {}
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
        let s = Session::new();
        let actual = s.read_journal_files();

        assert_eq!(1, actual);
    }
}
