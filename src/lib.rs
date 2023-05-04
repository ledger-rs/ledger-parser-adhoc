use session::Session;

/**
 * Ledger parser library.
 * The lib module is equivalent to global.cc
 */

mod account;
mod amount;
mod commodity;
mod context;
mod filters;
mod global;
mod journal;
mod new_parser;
mod output;
mod pool;
mod post;
mod report;
mod scope;
mod session;
mod textual;
mod utils;
mod xact;

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
