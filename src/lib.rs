/**
 * Ledger parser library.
 * The lib module is equivalent to global.cc
 */

use global::GlobalScope;

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

/// main function from Ledger
pub fn main(args: Vec<&str>) {
    log::info!("Ledger starting");

    // Create the session object, which maintains nearly all state relating to
    // this invocation of Ledger; and register all known journal parsers.
    
    // Global scope contains Session.
    // todo: global_scope = new global_scope_t(envp);
    let mut global_scope = GlobalScope::new();

    // TODO: read command arguments
    // let args = global_scope.read_command_arguments(args);

    // args = argv[i]
    // todo: if script handler
    // TODO: borrowing issues
    // if !args.is_empty() {
    //     global_scope.execute_command_wrapper(args);
    // }
}

#[cfg(test)]
mod tests {
    use crate::main;


    #[test]
    fn test_main() {
        let args = vec!["b", "-f", "tests/first.ledger"];
        main(args);

        assert!(false);
    }
}
