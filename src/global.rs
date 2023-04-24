use crate::session::Session;

/**
 * global.cc
 */

pub(crate) struct GlobalScope {
    session_ptr: Box<Session>,
}

impl GlobalScope {
    pub fn new() -> Self {
        GlobalScope {
            session_ptr: Box::new(Session::new()),
        }

        // set_session_context

        // if(!args_only)
        // read_environment_settings()
        // read_init()
    }

    fn read_init(&self) {
        todo!()
    }
}

/// Equivalent of the global context.
pub(crate) fn run() {
    log::info!("Ledger starting");

    let global_scope = GlobalScope::new();

    // todo: get command-line arguments
    // args = argv[i]

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
