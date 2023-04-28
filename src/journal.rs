use crate::{account::Account, context::{ParseContextStack, ParseContext}, textual::Instance};

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

    /// journal.cc,
    /// std::size_t journal_t::read(parse_context_stack_t &context)
    pub fn read(&self, context_stack: &ParseContextStack) -> usize {
        let mut count: usize = 0;

        count = read_textual(context_stack, context_stack.get_current());

        count
    }
    
}


fn read_textual(context_stack: &ParseContextStack, context: &ParseContext) -> usize {
    let instance: Instance = Instance::new(context_stack, context);
    // instance.apply_stack...
    instance.parse()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{context::ParseContextStack, account::Account};

    use super::Journal;

    #[test]
    fn test_journal_read() {
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