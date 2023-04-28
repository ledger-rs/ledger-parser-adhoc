use crate::{account::Account, context::{ParseContext, ParseContextStack}};

/**
 * journal.cc + journal.h
 */

pub(crate) struct Journal {
    pub master: Account,
    // bucket
    // xacts
    // sources
    // known_payees
    // known_tags

    // current_context: &'a ParseContext
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            master: Account::new(),
            // current_context: ,
          }
    }

    pub fn find_account(&self, name: &str, auto_create: bool) -> &Account {
        todo!()
    }

    /// journal.cc, #469
    pub fn read(&self, context: &ParseContextStack) -> usize {
        let mut count: usize = 0;

        read_textual()
    }
    
}


fn read_textual() -> usize {
    parse()
}

fn parse() -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{context::ParseContextStack, account::Account};

    use super::Journal;

    #[test]
    fn test_journal_parsing() {
        let journal = Journal::new();
        let master = &Account::new();
        let mut context = ParseContextStack::new();
        let pathname = PathBuf::from("tests/first.ledger");
        //context.push(&path);
        context.add_new(&pathname, &journal, master);

        let actual = journal.read(&context);

        assert!(false);
    }
}