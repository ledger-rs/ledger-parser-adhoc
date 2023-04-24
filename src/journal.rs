use crate::account::Account;

/**
 * journal.cc + journal.h
 */

pub(crate) struct Journal {
    master: Account
    // bucket
    // xacts
    // sources
    // known_payees
    // known_tags
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            master: Account::new()
          }
    }
}

fn read() {
    read_textual();
    // todo!("journal.cc, #469")
}

fn read_textual() {
    parse();
}

fn parse() {

}