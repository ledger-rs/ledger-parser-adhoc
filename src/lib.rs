use global::GlobalScope;
use session::Session;

/**
 * Ledger parser library.
 * The lib module is equivalent to global.cc
 */

mod account;
mod context;
mod global;
mod journal;
mod session;

pub fn parse() {
    // global::run();
    let mut s = Session::new();
    s.read_journal_files();
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn full_cycle() {
        parse();

        assert!(false);
    }
}
