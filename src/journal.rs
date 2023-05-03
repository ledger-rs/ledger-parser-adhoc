use crate::{
    account::Account,
    context::{ParseContext, ParseContextStack},
    textual::Instance,
    xact::Xact,
};

/**
 * journal.cc + journal.h
 */

pub struct Journal {
    pub master: Account,
    // bucket
    pub xacts: Vec<Xact>,
    // sources
    // known_payees
    // known_tags

    // current_context: &'a ParseContext
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            master: Account::new(),
            xacts: vec![],
            // current_context: ,
        }
    }

    pub fn add_xact(&mut self, xact: Xact) {
        // todo: xact.journal =
        // TODO: xact.finalize()

        // todo: extend_xact()
        // todo: check_all_metadata())
        // todo: for each post - extend + check metadata

        self.xacts.push(xact);
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
    let mut instance: Instance = Instance::new(context_stack, context);
    // instance.apply_stack...
    instance.parse()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{account::Account, context::ParseContextStack};

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
