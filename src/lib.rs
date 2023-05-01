use session::Session;

/**
 * Ledger parser library.
 * The lib module is equivalent to global.cc
 */

mod account;
mod amount;
mod commodity;
mod context;
mod global;
mod journal;
mod new_parser;
mod post;
mod session;
mod textual;
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
