use crate::scope::Kind;

/**
 * This is intended as the entry point for running the reports.
 */

/// Run a report
pub fn run(args: Vec<String>) {
    // todo: read journal files

    // todo: run report

}

/// expr_t::ptr_op_t report_t::lookup(const symbol_t::kind_t kind,
///     const string& name)
fn lookup(kind: Kind, name: &str) {
    let first_char = name.chars().next().expect("there has to be a command");

    match kind {
        Kind::UNKNOWN => todo!(),
        Kind::FUNCTION => todo!(),
        Kind::OPTION => todo!(),
        Kind::PRECOMMAND => todo!(),
        Kind::COMMAND => match first_char {
            'b' => {
                // check if balance, 'b', 'bal', 'balance'
                todo!("b commands")
            },
            _ => todo!("other commands")
        },
        Kind::DIRECTIVE => todo!(),
        Kind::FORMAT => todo!(),
    }
}

/// void report_t::accounts_report(acct_handler_ptr handler)
fn accounts_report() {
    // todo: chain post handlers
    // post splitter
    // pre-flush, post-flush

    // TODO: iterate over journal posts.
    // pass_down_posts()

    // accounts_flusher()
}

 #[cfg(test)]
 mod tests {
    use crate::report::run;

     /// development test, used for debugging through.
     #[test]
     fn test_bal_report() {
         let args = vec!["b".into()];
 
         let actual = run(args);
 
         assert!(false)
     }
 }