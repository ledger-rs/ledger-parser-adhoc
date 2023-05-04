use crate::session::Session;

/**
 * global.cc
 */

pub(crate) struct GlobalScope<'a> {
    session_ptr: Box<Session<'a>>,
}

impl <'a> GlobalScope<'a> {
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

    // TODO: get command-line arguments
    // args = argv[i]
    let args = vec![];

    // todo: if (!args.empty())
    execute_command_wrapper(args);
}

fn execute_command_wrapper(args: Vec<String>) {
    execute_command();
}

/// Line 196
fn execute_command() {
    // not pre-command
    // not repl
}

