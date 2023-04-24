use crate::journal::Journal;

/**
 * session.cc + session.h
 */

pub(crate) struct Session {
    journal: Journal,
}

impl Session {
    pub fn new() -> Self {
        Self {
            journal: Journal::new()
        }
    }
    fn read_journal_files(&self) -> &Journal {
        let count: usize = self.read_data();

        &self.journal
    }

    pub(crate) fn read_data(&self) -> usize {
        todo!("session.cc, line 72")

        // todo: get file paths
    }
}

#[cfg(test)]
mod tests {
    use crate::session::Session;

    #[test]
    fn test_basic() {
        let tx = r#"
2023-04-10 Supermarket
    Expenses:Food  20 EUR
    Assets:Cash
"#;
        let s = Session::new();
        let actual = s.read_journal_files();

        assert_eq!(1, actual);
    }
}
