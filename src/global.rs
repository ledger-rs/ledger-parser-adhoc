use crate::session::Session;

/**
 * global.cc
 */

pub(crate) struct GlobalScope<'a> {
    session_ptr: Session<'a>,
}

impl<'a> GlobalScope<'a> {
    pub fn new() -> Self {
        GlobalScope {
            session_ptr: Session::new(),
        }

        // todo: set_session_context()
        // todo: create report object?

        // Read the user's options, ...
        // if(!args_only)
        // read_environment_settings()
        // read_init()
    }

    fn read_init(&self) {
        todo!()
    }

    pub fn execute_command_wrapper(&mut self, args: Vec<String>) {
        // check if repl
    
        self.execute_command(args);
    }
    
    /// Line 196
    fn execute_command(&mut self, args: Vec<String>) {
        // pre-command?
    
        // not pre-command
        // not repl
        self.session_ptr.read_journal_files();

        // report.normalize_options

        // TODO: look for command

    }
    
    fn look_for_command(&self) {

    }
}
