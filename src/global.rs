use crate::session::Session;

/**
 * global.cc
 */

pub(crate) struct GlobalScope {
    session_ptr: Box<Session>
}

impl GlobalScope {
    pub fn new() -> Self {
        GlobalScope { session_ptr: todo!()  }
    }
}

/// Equivalent of the global context.
pub(crate) fn run() {
  // log::info!("Ledger starting");

    let global_scope = GlobalScope::new();

    // todo: get command-line arguments

    // todo: if (!args.empty())
    execute_command_wrapper();
}

fn execute_command_wrapper() {
    execute_command();
}

/// Line 196
fn execute_command() {
    // not pre-command
    // not repl

}