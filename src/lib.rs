use global::GlobalScope;

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
    global::run();
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
