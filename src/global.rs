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
        todo!("implement")
    }

    pub fn execute_command_wrapper(&'a mut self, args: Vec<&str>) {
        // check if repl
    
        self.execute_command(args);
    }
    
    /// Line 196
    fn execute_command(&'a mut self, args: Vec<&str>) {
        // pre-command?
    
        // not pre-command
        // not repl
        self.session_ptr.read_journal_files();

        // report.normalize_options

        // TODO: look for command

    }
    
    fn look_for_command(&self) {
        todo!("continue")
    }

    /// Takes the command arguments from the list of arguments.
    /// i.e. for "b -f test.ledger", it takes the "-f" and leaves only ["b"].
    pub fn read_command_arguments(&self, args: Vec<&str>) -> Vec<&str> {
        let remaining = self.process_arguments(args);

        remaining
    }

    fn process_arguments(&self, args: Vec<&str>) -> Vec<&str> {
        // TODO: identify arguments and their values
        let argument = "";
        let value = "";

        self.process_option(argument, value);

        todo!()
    }

    fn process_option(&self, whence: &str, value: &str) {
        todo!()
    }
}
