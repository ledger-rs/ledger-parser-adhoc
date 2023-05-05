/**
 * Ledger parser library.
 * The lib module is equivalent to global.cc
 */

use global::GlobalScope;
use session::Session;

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

/// main function from Ledger
pub fn main(args: Vec<String>) {
    log::info!("Ledger starting");

    // Create the session object, which maintains nearly all state relating to
    // this invocation of Ledger; and register all known journal parsers.
    
    // Global scope contains Session.
    // todo: global_scope = new global_scope_t(envp);
    let mut global_scope = GlobalScope::new();

    // todo: read command arguments
    // args = argv[i]
    // todo: if script handler
    if !args.is_empty() {
        // TODO: global_scope.execute
        global_scope.execute_command_wrapper(args);
    }
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
